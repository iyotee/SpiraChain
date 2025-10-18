// LibP2P Network with Full Block Synchronization
// SIMPLE implementation: Gossipsub for broadcast + manual block requests

use libp2p::{
    gossipsub,
    identity::Keypair,
    noise,
    swarm::{Swarm, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId,
};
use spirachain_core::{Block, Result, SpiraChainError, Transaction};
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn};

use crate::bootstrap::{discover_bootstrap_peers, BootstrapConfig};

pub struct LibP2PNetworkWithSync {
    swarm: Swarm<gossipsub::Behaviour>,
    #[allow(dead_code)]
    local_peer_id: PeerId,
    connected_peers: HashSet<PeerId>,
    block_topic: gossipsub::IdentTopic,
    tx_topic: gossipsub::IdentTopic,
    sync_topic: gossipsub::IdentTopic, // For height announcements
    is_listening: bool,
    listen_port: u16,
    network: String,
    local_height: u64,
    last_height_announcement: std::time::Instant,
    bootstrap_addrs: Vec<Multiaddr>, // Store bootstrap addresses for reconnection
    last_reconnect_attempt: std::time::Instant,
    peer_heights: HashMap<PeerId, u64>, // Track peer heights
}

// Network events
#[derive(Debug)]
pub enum NetworkEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    PeerHeight { peer: PeerId, height: u64 },
    NewBlock(Block),
    NewTransaction(Transaction),
    BlockRequested(u64), // A peer requested a specific block height
}

impl LibP2PNetworkWithSync {
    pub async fn new(port: u16, local_height: u64) -> Result<Self> {
        Self::new_with_network(port, "testnet", local_height).await
    }

    pub async fn new_with_network(port: u16, network: &str, local_height: u64) -> Result<Self> {
        info!("🌐 Initializing LibP2P Network with block sync");
        info!("   Network: {}", network.to_uppercase());
        info!("   Local Height: {}", local_height);

        // Generate keypair
        let local_key = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        info!("   Local PeerID: {}", local_peer_id);

        // Create Gossipsub
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .build()
            .map_err(|e| SpiraChainError::NetworkError(format!("Gossipsub config: {}", e)))?;

        let behaviour = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .map_err(|e| SpiraChainError::NetworkError(format!("Gossipsub init: {}", e)))?;

        // Create Swarm
        let swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|e| SpiraChainError::NetworkError(format!("TCP transport: {}", e)))?
            .with_behaviour(|_key| behaviour)
            .map_err(|e| SpiraChainError::NetworkError(format!("Behaviour: {}", e)))?
            .with_swarm_config(|c| {
                c.with_idle_connection_timeout(std::time::Duration::from_secs(300)) // 5 minutes
            })
            .build();

        let block_topic = gossipsub::IdentTopic::new("spirachain-blocks");
        let tx_topic = gossipsub::IdentTopic::new("spirachain-transactions");
        let sync_topic = gossipsub::IdentTopic::new("spirachain-sync");

        info!("✅ P2P network initialized with Gossipsub");

        Ok(Self {
            swarm,
            local_peer_id,
            connected_peers: HashSet::new(),
            block_topic,
            tx_topic,
            sync_topic,
            is_listening: false,
            listen_port: port,
            network: network.to_string(),
            local_height,
            last_height_announcement: std::time::Instant::now(),
            bootstrap_addrs: Vec::new(),
            last_reconnect_attempt: std::time::Instant::now(),
            peer_heights: HashMap::new(),
        })
    }

    /// Placeholder for block store callback (not needed with simple gossipsub)
    pub fn set_block_store_callback<F>(&mut self, _callback: F)
    where
        F: Fn(Block) -> Result<()> + Send + Sync + 'static,
    {
        // Not used in simple implementation
    }

    /// Initialize P2P network with bootstrap discovery
    pub async fn initialize_with_bootstrap(&mut self) -> Result<()> {
        if self.is_listening {
            return Ok(());
        }

        // Listen on all interfaces
        let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", self.listen_port)
            .parse()
            .map_err(|e| SpiraChainError::NetworkError(format!("Invalid addr: {}", e)))?;

        self.swarm
            .listen_on(listen_addr.clone())
            .map_err(|e| SpiraChainError::NetworkError(format!("Listen failed: {}", e)))?;

        info!("📡 Listening on: {}", listen_addr);

        // Subscribe to topics
        self.swarm
            .behaviour_mut()
            .subscribe(&self.block_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe blocks: {}", e)))?;
        self.swarm
            .behaviour_mut()
            .subscribe(&self.tx_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe tx: {}", e)))?;
        self.swarm
            .behaviour_mut()
            .subscribe(&self.sync_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe sync: {}", e)))?;

        info!("✅ Subscribed to topics: blocks, transactions, sync");

        // Discover bootstrap peers
        info!("🔍 Discovering bootstrap peers...");
        let config = BootstrapConfig::for_network(&self.network);
        match discover_bootstrap_peers(&config).await {
            Ok(bootstrap_peers) => {
                if bootstrap_peers.is_empty() {
                    warn!("⚠️  No bootstrap peers found - running in isolated mode");
                } else {
                    info!("📋 Found {} bootstrap peers", bootstrap_peers.len());

                    let mut dialed_count = 0;
                    for addr_str in &bootstrap_peers {
                        if let Ok(addr) = addr_str.parse::<Multiaddr>() {
                            // Store bootstrap addresses for reconnection
                            self.bootstrap_addrs.push(addr.clone());
                            
                            // Try to dial - if it fails with "Broken pipe", it's probably ourselves
                            // LibP2P will automatically prevent self-dial
                            match self.swarm.dial(addr.clone()) {
                                Ok(_) => {
                                    info!("📞 Dialing: {}", addr);
                                    dialed_count += 1;
                                }
                                Err(e) => {
                                    // Silently skip dial errors (likely self-dial)
                                    debug!("⊘ Skipping {}: {}", addr, e);
                                }
                            }
                        }
                    }

                    if dialed_count == 0 {
                        info!("⚠️  No external peers to dial - running as first node");
                        info!("   Waiting for incoming connections...");
                    } else {
                        info!("✅ Dialing {} peers...", dialed_count);
                    }
                }
            }
            Err(e) => {
                warn!(
                    "⚠️  Failed to discover bootstrap peers: {}. Running in isolated mode.",
                    e
                );
            }
        }

        self.is_listening = true;

        // Announce our height
        self.announce_height();

        Ok(())
    }

    /// Update local blockchain height
    pub fn set_local_height(&mut self, height: u64) {
        let height_changed = height != self.local_height;
        self.local_height = height;

        // Announce height every 10 seconds (keep-alive) OR if changed
        let elapsed = self.last_height_announcement.elapsed();
        if height_changed || elapsed.as_secs() >= 10 {
            self.announce_height();
            self.last_height_announcement = std::time::Instant::now();
        }
    }

    /// Announce our blockchain height to peers
    fn announce_height(&mut self) {
        let msg = format!("HEIGHT:{}", self.local_height);
        let data = msg.as_bytes().to_vec();
        if let Err(e) = self
            .swarm
            .behaviour_mut()
            .publish(self.sync_topic.clone(), data)
        {
            debug!("Failed to announce height: {}", e);
        } else {
            debug!("📢 Announced height: {}", self.local_height);
        }
    }

    /// Poll for network events (non-blocking)
    pub async fn poll_events(&mut self) -> Option<NetworkEvent> {
        // Use poll_next instead of select_next_some to avoid blocking
        use futures::stream::StreamExt;
        
        match futures::poll!(self.swarm.next()) {
            std::task::Poll::Ready(Some(event)) => match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("📡 Listening on: {}", address);
                None
            }
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                info!(
                    "🤝 Connected to peer: {} at {}",
                    peer_id,
                    endpoint.get_remote_address()
                );
                self.connected_peers.insert(peer_id);

                // Announce our height to new peer
                self.announce_height();

                Some(NetworkEvent::PeerConnected(peer_id))
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                info!("👋 Disconnected from peer: {}", peer_id);
                self.connected_peers.remove(&peer_id);
                self.peer_heights.remove(&peer_id);
                
                // Schedule reconnection attempt
                self.last_reconnect_attempt = std::time::Instant::now();
                
                Some(NetworkEvent::PeerDisconnected(peer_id))
            }
            SwarmEvent::Behaviour(gossip_event) => self.handle_gossipsub_event(gossip_event),
            _ => None,
            }
            std::task::Poll::Ready(None) => None,
            std::task::Poll::Pending => None, // No event ready, return None immediately
        }
    }

    fn handle_gossipsub_event(&mut self, event: gossipsub::Event) -> Option<NetworkEvent> {
        match event {
            gossipsub::Event::Message { message, .. } => {
                if message.topic == self.block_topic.hash() {
                    // Received a new block
                    match bincode::deserialize::<Block>(&message.data) {
                        Ok(block) => {
                            info!(
                                "📦 Received new block {} via gossip",
                                block.header.block_height
                            );
                            Some(NetworkEvent::NewBlock(block))
                        }
                        Err(e) => {
                            warn!("Failed to deserialize block: {}", e);
                            None
                        }
                    }
                } else if message.topic == self.tx_topic.hash() {
                    // Received a new transaction
                    match bincode::deserialize::<Transaction>(&message.data) {
                        Ok(tx) => {
                            debug!("📨 Received new transaction via gossip");
                            Some(NetworkEvent::NewTransaction(tx))
                        }
                        Err(e) => {
                            warn!("Failed to deserialize transaction: {}", e);
                            None
                        }
                    }
                } else if message.topic == self.sync_topic.hash() {
                    // Received sync message (height announcement or block request)
                    if let Ok(msg) = String::from_utf8(message.data.clone()) {
                        if let Some(height_str) = msg.strip_prefix("HEIGHT:") {
                            if let Ok(peer_height) = height_str.parse::<u64>() {
                                // Track peer height
                                if let Some(propagation_source) = message.source {
                                    self.peer_heights.insert(propagation_source, peer_height);
                                    info!("📊 Peer {} at height: {}", propagation_source, peer_height);
                                }

                                // If peer is ahead, we're behind and need to catch up
                                if peer_height > self.local_height {
                                    let blocks_behind = peer_height - self.local_height;
                                    info!(
                                        "🔄 We are {} blocks behind (peer at {}, us at {})",
                                        blocks_behind, peer_height, self.local_height
                                    );

                                    // Request missing blocks in batches of 50
                                    let batch_size = 50;
                                    let start = self.local_height + 1;
                                    let end = std::cmp::min(start + batch_size - 1, peer_height);

                                    let request_msg = format!("GET_BLOCKS:{}-{}", start, end);
                                    info!(
                                        "📥 Requesting blocks {} to {} (batch of {})",
                                        start,
                                        end,
                                        end - start + 1
                                    );

                                    if let Err(e) = self.swarm.behaviour_mut().publish(
                                        self.sync_topic.clone(),
                                        request_msg.as_bytes().to_vec(),
                                    ) {
                                        warn!("Failed to request blocks: {}", e);
                                    }
                                }
                                None
                            } else {
                                None
                            }
                        } else if msg.starts_with("GET_BLOCKS:") {
                            // Someone is requesting a range of blocks
                            // Format: GET_BLOCKS:start-end
                            if let Some(range_str) = msg.strip_prefix("GET_BLOCKS:") {
                                if let Some((start_str, _end_str)) = range_str.split_once('-') {
                                    if let Ok(start) = start_str.parse::<u64>() {
                                        info!("📤 Peer requested blocks starting at {}", start);
                                        // ValidatorNode will handle sending the range
                                        return Some(NetworkEvent::BlockRequested(start));
                                    }
                                }
                            }
                            None
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Broadcast a block via Gossipsub
    pub async fn broadcast_block(&mut self, block: &Block) -> Result<()> {
        let data = bincode::serialize(block)
            .map_err(|e| SpiraChainError::SerializationError(e.to_string()))?;

        self.swarm
            .behaviour_mut()
            .publish(self.block_topic.clone(), data)
            .map_err(|e| SpiraChainError::NetworkError(format!("Broadcast block: {}", e)))?;

        debug!("📡 Broadcasted block {}", block.header.block_height);
        Ok(())
    }

    /// Send a specific block (in response to GET_BLOCK request)
    pub async fn send_block(&mut self, block: &Block) -> Result<()> {
        let data = bincode::serialize(block)
            .map_err(|e| SpiraChainError::SerializationError(e.to_string()))?;

        self.swarm
            .behaviour_mut()
            .publish(self.block_topic.clone(), data)
            .map_err(|e| SpiraChainError::NetworkError(format!("Send block: {}", e)))?;

        info!("📤 Sent block {} to peers", block.header.block_height);
        Ok(())
    }

    /// Broadcast a transaction via Gossipsub
    pub async fn broadcast_transaction(&mut self, tx: &Transaction) -> Result<()> {
        let data = bincode::serialize(tx)
            .map_err(|e| SpiraChainError::SerializationError(e.to_string()))?;

        self.swarm
            .behaviour_mut()
            .publish(self.tx_topic.clone(), data)
            .map_err(|e| SpiraChainError::NetworkError(format!("Broadcast tx: {}", e)))?;

        debug!("📨 Broadcasted transaction");
        Ok(())
    }

    /// Get connected peer count
    pub fn peer_count(&self) -> usize {
        self.connected_peers.len()
    }
    
    /// Get connected peer count (alias for compatibility)
    pub fn connected_peers_count(&self) -> usize {
        self.connected_peers.len()
    }
    
    /// Get peer heights map (for sync checking)
    pub fn get_peer_heights(&self) -> &HashMap<PeerId, u64> {
        &self.peer_heights
    }

    /// Get sync statistics (simplified)
    pub fn get_sync_stats(&self) -> String {
        format!(
            "Height: {} | Peers: {}",
            self.local_height,
            self.peer_count()
        )
    }

    /// Check if node is synced (always true for simple gossipsub)
    pub fn is_synced(&self) -> bool {
        true // Gossipsub doesn't have sync state
    }

    /// Attempt to reconnect to bootstrap peers if disconnected
    pub fn try_reconnect(&mut self) {
        // Only try reconnection every 30 seconds
        if self.last_reconnect_attempt.elapsed().as_secs() < 30 {
            return;
        }

        // If we have no connected peers, try to reconnect to bootstrap peers
        if self.connected_peers.is_empty() && !self.bootstrap_addrs.is_empty() {
            // Get our listening addresses to filter out self-dial attempts
            let our_addrs: Vec<Multiaddr> = self.swarm.listeners().cloned().collect();
            
            let mut attempted = 0;
            for addr in &self.bootstrap_addrs {
                // Skip if this is one of our own listening addresses
                let is_self = our_addrs.iter().any(|our_addr| {
                    // Compare IP and port
                    addr.to_string().contains(&our_addr.to_string())
                });
                
                if is_self {
                    debug!("⊘ Skipping self-dial: {}", addr);
                    continue;
                }
                
                match self.swarm.dial(addr.clone()) {
                    Ok(_) => {
                        debug!("📞 Reconnecting to: {}", addr);
                        attempted += 1;
                    }
                    Err(e) => {
                        debug!("⊘ Reconnect failed for {}: {}", addr, e);
                    }
                }
            }
            
            if attempted > 0 {
                info!("🔄 Attempting reconnection to {} bootstrap peers...", attempted);
            } else {
                debug!("⊘ No external peers to reconnect to (all are self or unavailable)");
            }
            
            self.last_reconnect_attempt = std::time::Instant::now();
        }
    }
}
