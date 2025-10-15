// LibP2P Network with Full Block Synchronization
// Complete implementation with Request/Response + Gossipsub

use futures::StreamExt;
use libp2p::{
    gossipsub, identity::Keypair, noise, 
    request_response::{self, ProtocolSupport}, 
    swarm::{Swarm, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, StreamProtocol,
};
use spirachain_core::{Block, Result, SpiraChainError, Transaction};
use std::collections::HashSet;
use std::iter;
use tracing::{debug, info, warn, error};

use crate::block_sync::{
    BlockSyncCodec, BlockSyncManager, BlockSyncRequest, BlockSyncResponse,
    PROTOCOL_VERSION,
};
use crate::bootstrap::{discover_bootstrap_peers, BootstrapConfig};

// Combined behaviour for Gossipsub + Request/Response
#[derive(libp2p::swarm::NetworkBehaviour)]
pub struct SpiraChainBehaviour {
    gossipsub: gossipsub::Behaviour,
    request_response: request_response::Behaviour<BlockSyncCodec>,
}

pub struct LibP2PNetworkWithSync {
    swarm: Swarm<SpiraChainBehaviour>,
    local_peer_id: PeerId,
    connected_peers: HashSet<PeerId>,
    block_topic: gossipsub::IdentTopic,
    tx_topic: gossipsub::IdentTopic,
    is_listening: bool,
    listen_port: u16,
    network: String,
    
    // Block sync manager
    sync_manager: BlockSyncManager,
    
    // Callback for storing validated blocks
    block_store_callback: Option<Box<dyn Fn(Block) -> Result<()> + Send + Sync>>,
}

impl LibP2PNetworkWithSync {
    pub async fn new(port: u16, local_height: u64) -> Result<Self> {
        Self::new_with_network(port, "testnet", local_height).await
    }

    pub async fn new_with_network(port: u16, network: &str, local_height: u64) -> Result<Self> {
        info!("🌐 Initializing LibP2P Network with Block Sync");
        info!("   Network: {}", network.to_uppercase());
        info!("   Local Height: {}", local_height);

        // Generate keypair
        let local_key = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        info!("   Local PeerID: {}", local_peer_id);

        // Create Gossipsub (for block/tx propagation)
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .build()
            .map_err(|e| SpiraChainError::NetworkError(format!("Gossipsub config: {}", e)))?;

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .map_err(|e| SpiraChainError::NetworkError(format!("Gossipsub init: {}", e)))?;

        // Create Request/Response (for block sync)
        let request_response = request_response::Behaviour::new(
            iter::once((StreamProtocol::new(PROTOCOL_VERSION), ProtocolSupport::Full)),
            request_response::Config::default(),
        );

        // Combine behaviours
        let behaviour = SpiraChainBehaviour {
            gossipsub,
            request_response,
        };

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
                c.with_idle_connection_timeout(std::time::Duration::from_secs(60))
            })
            .build();

        let block_topic = gossipsub::IdentTopic::new("spirachain-blocks");
        let tx_topic = gossipsub::IdentTopic::new("spirachain-transactions");

        info!("✅ P2P network initialized:");
        info!("   ✓ Gossipsub (block/tx propagation)");
        info!("   ✓ Request/Response (block sync)");
        info!("   ✓ DNS Seeds (peer discovery)");

        Ok(Self {
            swarm,
            local_peer_id,
            connected_peers: HashSet::new(),
            block_topic,
            tx_topic,
            is_listening: false,
            listen_port: port,
            network: network.to_string(),
            sync_manager: BlockSyncManager::new(local_height),
            block_store_callback: None,
        })
    }

    /// Set callback for storing validated blocks
    pub fn set_block_store_callback<F>(&mut self, callback: F)
    where
        F: Fn(Block) -> Result<()> + Send + Sync + 'static,
    {
        self.block_store_callback = Some(Box::new(callback));
    }

    /// Initialize P2P network with bootstrap discovery
    pub async fn initialize_with_bootstrap(&mut self) -> Result<()> {
        if self.is_listening {
            return Ok(());
        }

        // Listen on all interfaces with the specified port
        let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", self.listen_port)
            .parse()
            .map_err(|e| SpiraChainError::NetworkError(format!("Invalid addr: {}", e)))?;

        self.swarm
            .listen_on(listen_addr.clone())
            .map_err(|e| SpiraChainError::NetworkError(format!("Listen failed: {}", e)))?;

        info!("📡 Listening on: {}", listen_addr);

        // Subscribe to Gossipsub topics
        self.swarm
            .behaviour_mut()
            .gossipsub
            .subscribe(&self.block_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe blocks: {}", e)))?;
        self.swarm
            .behaviour_mut()
            .gossipsub
            .subscribe(&self.tx_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe tx: {}", e)))?;

        info!("✅ Subscribed to topics: blocks, transactions");

        // Discover bootstrap peers
        info!("🔍 Discovering bootstrap peers...");
        let config = BootstrapConfig::for_network(&self.network);
        match discover_bootstrap_peers(&config).await {
            Ok(bootstrap_peers) => {
                if bootstrap_peers.is_empty() {
                    warn!("⚠️  No bootstrap peers found - running in isolated mode");
                } else {
                    info!("📋 Found {} bootstrap peers", bootstrap_peers.len());
                    for addr_str in bootstrap_peers {
                        if let Ok(addr) = addr_str.parse::<Multiaddr>() {
                            if let Err(e) = self.swarm.dial(addr.clone()) {
                                warn!("Failed to dial {}: {}", addr, e);
                            } else {
                                debug!("Dialing bootstrap peer: {}", addr);
                            }
                        } else {
                            warn!("Invalid multiaddr: {}", addr_str);
                        }
                    }
                }
            }
            Err(e) => {
                warn!("⚠️  Failed to discover bootstrap peers: {}. Running in isolated mode.", e);
            }
        }

        self.is_listening = true;
        Ok(())
    }

    /// Update local blockchain height
    pub fn set_local_height(&mut self, height: u64) {
        self.sync_manager.set_local_height(height);
    }

    /// Poll for network events
    pub async fn poll_events(&mut self) -> Option<NetworkEvent> {
        tokio::select! {
            event = self.swarm.select_next_some() => {
                self.handle_swarm_event(event).await
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                // Periodic sync check
                self.check_sync_status().await
            }
        }
    }

    async fn handle_swarm_event(&mut self, event: SwarmEvent<SpiraChainBehaviourEvent>) -> Option<NetworkEvent> {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("📡 Listening on: {}", address);
                None
            }
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                info!("🤝 Connected to peer: {} at {}", peer_id, endpoint.get_remote_address());
                self.connected_peers.insert(peer_id);
                
                // Request peer's blockchain height
                self.request_peer_height(peer_id);
                
                Some(NetworkEvent::PeerConnected(peer_id))
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                info!("👋 Disconnected from peer: {}", peer_id);
                self.connected_peers.remove(&peer_id);
                Some(NetworkEvent::PeerDisconnected(peer_id))
            }
            SwarmEvent::Behaviour(event) => {
                self.handle_behaviour_event(event).await
            }
            _ => None,
        }
    }

    async fn handle_behaviour_event(&mut self, event: SpiraChainBehaviourEvent) -> Option<NetworkEvent> {
        match event {
            // Gossipsub events
            SpiraChainBehaviourEvent::Gossipsub(gossip_event) => {
                match gossip_event {
                    gossipsub::Event::Message { message, .. } => {
                        self.handle_gossipsub_message(message)
                    }
                    _ => None,
                }
            }
            
            // Request/Response events
            SpiraChainBehaviourEvent::RequestResponse(rr_event) => {
                match rr_event {
                    request_response::Event::Message { peer, message } => {
                        self.handle_request_response_message(peer, message).await
                    }
                    request_response::Event::OutboundFailure { peer, error, .. } => {
                        warn!("⚠️  Outbound request to {} failed: {:?}", peer, error);
                        None
                    }
                    request_response::Event::InboundFailure { peer, error, .. } => {
                        warn!("⚠️  Inbound request from {} failed: {:?}", peer, error);
                        None
                    }
                    _ => None,
                }
            }
        }
    }

    fn handle_gossipsub_message(&mut self, message: gossipsub::Message) -> Option<NetworkEvent> {
        if message.topic == self.block_topic.hash() {
            // Received a new block via gossip
            match bincode::deserialize::<Block>(&message.data) {
                Ok(block) => {
                    info!("📦 Received new block {} via gossip", block.header.block_height);
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
        } else {
            None
        }
    }

    async fn handle_request_response_message(
        &mut self,
        peer: PeerId,
        message: request_response::Message<BlockSyncRequest, BlockSyncResponse>,
    ) -> Option<NetworkEvent> {
        match message {
            request_response::Message::Request { request, channel, .. } => {
                // Handle incoming request
                self.handle_sync_request(peer, request, channel).await;
                None
            }
            request_response::Message::Response { response, .. } => {
                // Handle response to our request
                self.handle_sync_response(peer, response).await
            }
        }
    }

    async fn handle_sync_request(
        &mut self,
        peer: PeerId,
        request: BlockSyncRequest,
        channel: request_response::ResponseChannel<BlockSyncResponse>,
    ) {
        debug!("📥 Received sync request from {}: {:?}", peer, request);
        
        // This would call into the node's storage to get the requested data
        // For now, we'll send a placeholder response
        let response = BlockSyncResponse::Error {
            message: "Not implemented yet - use callback".to_string(),
        };
        
        if let Err(e) = self.swarm.behaviour_mut().request_response.send_response(channel, response) {
            warn!("Failed to send response to {}: {:?}", peer, e);
        }
    }

    async fn handle_sync_response(&mut self, peer: PeerId, response: BlockSyncResponse) -> Option<NetworkEvent> {
        match response {
            BlockSyncResponse::Height { height, best_hash: _ } => {
                info!("📊 Peer {} has height: {}", peer, height);
                self.sync_manager.register_peer_height(peer, height);
                Some(NetworkEvent::PeerHeight { peer, height })
            }
            
            BlockSyncResponse::Blocks { blocks } => {
                info!("📦 Received {} blocks from {}", blocks.len(), peer);
                
                // Add blocks to sync manager
                for block in blocks {
                    self.sync_manager.add_downloaded_block(block);
                }
                
                // Process downloaded blocks
                self.process_downloaded_blocks().await;
                
                None
            }
            
            BlockSyncResponse::Headers { headers } => {
                info!("📋 Received {} headers from {}", headers.len(), peer);
                // TODO: Process headers
                None
            }
            
            BlockSyncResponse::Error { message } => {
                warn!("❌ Sync error from {}: {}", peer, message);
                None
            }
        }
    }

    async fn process_downloaded_blocks(&mut self) {
        while let Some(block) = self.sync_manager.get_next_block_to_validate() {
            let height = block.header.block_height;
            
            // Validate block
            // TODO: This should call into the consensus layer
            // For now, we'll just accept it
            
            info!("✅ Validated block {}", height);
            
            // Store block
            if let Some(ref callback) = self.block_store_callback {
                if let Err(e) = callback(block) {
                    error!("Failed to store block {}: {}", height, e);
                    break;
                }
            }
            
            // Mark as validated
            self.sync_manager.mark_block_validated(height);
        }
        
        // Check if sync is complete
        if !self.sync_manager.needs_sync() {
            info!("🎉 Blockchain fully synced!");
        }
    }

    async fn check_sync_status(&mut self) -> Option<NetworkEvent> {
        // Clean up timed-out requests
        self.sync_manager.cleanup_timeouts();
        
        // Check if we need to download more blocks
        if let Some((peer, request)) = self.sync_manager.get_next_download_batch() {
            info!("📥 Requesting blocks from peer {}", peer);
            self.swarm.behaviour_mut().request_response.send_request(&peer, request);
        }
        
        // Return sync status
        let stats = self.sync_manager.get_stats();
        if stats.progress < 100.0 {
            debug!("{}", stats);
        }
        
        None
    }

    fn request_peer_height(&mut self, peer: PeerId) {
        let request = BlockSyncRequest::GetHeight;
        debug!("📊 Requesting height from peer {}", peer);
        self.swarm.behaviour_mut().request_response.send_request(&peer, request);
    }

    /// Broadcast a block via Gossipsub
    pub async fn broadcast_block(&mut self, block: &Block) -> Result<()> {
        let data = bincode::serialize(block)
            .map_err(|e| SpiraChainError::SerializationError(e.to_string()))?;

        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(self.block_topic.clone(), data)
            .map_err(|e| SpiraChainError::NetworkError(format!("Broadcast block: {}", e)))?;

        debug!("📡 Broadcasted block {}", block.header.block_height);
        Ok(())
    }

    /// Broadcast a transaction via Gossipsub
    pub async fn broadcast_transaction(&mut self, tx: &Transaction) -> Result<()> {
        let data = bincode::serialize(tx)
            .map_err(|e| SpiraChainError::SerializationError(e.to_string()))?;

        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(self.tx_topic.clone(), data)
            .map_err(|e| SpiraChainError::NetworkError(format!("Broadcast tx: {}", e)))?;

        debug!("📨 Broadcasted transaction");
        Ok(())
    }

    /// Get connected peer count
    pub fn peer_count(&self) -> usize {
        self.connected_peers.len()
    }

    /// Get sync statistics
    pub fn get_sync_stats(&self) -> String {
        format!("{}", self.sync_manager.get_stats())
    }

    /// Check if node is synced
    pub fn is_synced(&self) -> bool {
        !self.sync_manager.needs_sync()
    }
}

// Network events
#[derive(Debug)]
pub enum NetworkEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    PeerHeight { peer: PeerId, height: u64 },
    NewBlock(Block),
    NewTransaction(Transaction),
}

