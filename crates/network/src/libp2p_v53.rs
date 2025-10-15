// LibP2P v0.53 Implementation for SpiraChain
// Using Gossipsub + DNS Seeds (simple, stable approach)

use futures::StreamExt;
use libp2p::{
    gossipsub,
    swarm::{Swarm, SwarmEvent},
    identity::Keypair,
    noise,
    tcp, yamux, Multiaddr, PeerId,
};
use spirachain_core::{Block, Result, SpiraChainError, Transaction};
use std::collections::HashSet;
use tracing::{debug, info, warn};

use crate::bootstrap::{discover_bootstrap_peers, BootstrapConfig};

pub struct LibP2PNetwork {
    swarm: Swarm<gossipsub::Behaviour>,
    local_peer_id: PeerId,
    connected_peers: HashSet<PeerId>,
    block_topic: gossipsub::IdentTopic,
    tx_topic: gossipsub::IdentTopic,
    is_listening: bool,
    listen_port: u16,
    network: String, // "testnet" or "mainnet"
}

impl LibP2PNetwork {
    pub async fn new(port: u16) -> Result<Self> {
        Self::new_with_network(port, "testnet").await
    }

    pub async fn new_with_network(port: u16, network: &str) -> Result<Self> {
        info!("🌐 Initializing LibP2P Network (Full P2P Stack)");
        info!("   Network: {}", network.to_uppercase());

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
                c.with_idle_connection_timeout(std::time::Duration::from_secs(60))
            })
            .build();

        let block_topic = gossipsub::IdentTopic::new("spirachain-blocks");
        let tx_topic = gossipsub::IdentTopic::new("spirachain-transactions");

        info!("✅ P2P network initialized:");
        info!("   ✓ Gossipsub (block/tx propagation)");
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
        })
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
            .listen_on(listen_addr)
            .map_err(|e| SpiraChainError::NetworkError(format!("Listen failed: {}", e)))?;

        // Subscribe to Gossipsub topics
        self.swarm
            .behaviour_mut()
            .subscribe(&self.block_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe blocks: {}", e)))?;
        self.swarm
            .behaviour_mut()
            .subscribe(&self.tx_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe tx: {}", e)))?;

        self.is_listening = true;
        info!("✅ P2P network listening on port {}", self.listen_port);

        // Discover and connect to bootstrap peers
        info!("🔍 Discovering bootstrap peers for {}...", self.network.to_uppercase());
        let config = BootstrapConfig::for_network(&self.network);

        match discover_bootstrap_peers(&config).await {
            Ok(peers) => {
                info!("📡 Found {} bootstrap peers", peers.len());
                for peer_addr in peers {
                    if let Ok(addr) = peer_addr.parse::<Multiaddr>() {
                        info!("   Connecting to: {}", addr);
                        
                        // Dial the peer
                        if let Err(e) = self.swarm.dial(addr.clone()) {
                            warn!("   Failed to dial {}: {}", addr, e);
                        }
                    }
                }
            }
            Err(e) => {
                warn!("⚠️  Bootstrap discovery failed: {}", e);
                warn!("   This is normal if DNS seeds are not yet configured");
                warn!("   Node will work independently until peers are discovered");
            }
        }

        Ok(())
    }

    /// Initialize P2P network (call once at startup) - legacy method
    pub fn initialize(&mut self) -> Result<()> {
        if self.is_listening {
            return Ok(());
        }

        // Listen on all interfaces
        let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0"
            .parse()
            .map_err(|e| SpiraChainError::NetworkError(format!("Invalid addr: {}", e)))?;

        self.swarm
            .listen_on(listen_addr)
            .map_err(|e| SpiraChainError::NetworkError(format!("Listen failed: {}", e)))?;

        // Subscribe to Gossipsub topics
        self.swarm
            .behaviour_mut()
            .subscribe(&self.block_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe blocks: {}", e)))?;
        self.swarm
            .behaviour_mut()
            .subscribe(&self.tx_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe tx: {}", e)))?;

        self.is_listening = true;
        info!("✅ P2P network listening initialized");

        Ok(())
    }

    /// Poll network events (call in validator loop)
    pub fn poll_events(&mut self) -> Option<String> {
        // Non-blocking poll
        match self
            .swarm
            .poll_next_unpin(&mut std::task::Context::from_waker(
                futures::task::noop_waker_ref(),
            )) {
            std::task::Poll::Ready(Some(event)) => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("📡 Listening on: {}", address);
                    Some(format!("Listening: {}", address))
                }
                SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                    info!("🤝 Connected to peer: {} at {}", peer_id, endpoint.get_remote_address());
                    self.connected_peers.insert(peer_id);
                    Some(format!("Connected: {}", peer_id))
                }
                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                    info!("👋 Disconnected from peer: {}", peer_id);
                    self.connected_peers.remove(&peer_id);
                    None
                }
                SwarmEvent::Behaviour(gossip_event) => {
                    self.handle_gossipsub_event(gossip_event);
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        // Listen on all interfaces with specified port
        let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", 0) // 0 = auto-assign
            .parse()
            .map_err(|e| SpiraChainError::NetworkError(format!("Invalid addr: {}", e)))?;

        self.swarm
            .listen_on(listen_addr)
            .map_err(|e| SpiraChainError::NetworkError(format!("Listen failed: {}", e)))?;

        // Subscribe to Gossipsub topics
        let block_topic = gossipsub::IdentTopic::new("spirachain-blocks");
        let tx_topic = gossipsub::IdentTopic::new("spirachain-transactions");

        self.swarm
            .behaviour_mut()
            .subscribe(&block_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe blocks: {}", e)))?;
        self.swarm
            .behaviour_mut()
            .subscribe(&tx_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe tx: {}", e)))?;

        info!("✅ P2P network listening");
        info!("   Topics: spirachain-blocks, spirachain-transactions");

        // Event loop
        loop {
            if let Some(event) = self.swarm.next().await {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        info!("📡 Listening on: {}", address);
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        info!("🤝 Connected to peer: {}", peer_id);
                        self.connected_peers.insert(peer_id);
                    }
                    SwarmEvent::ConnectionClosed { peer_id, .. } => {
                        info!("👋 Disconnected from peer: {}", peer_id);
                        self.connected_peers.remove(&peer_id);
                    }
                    SwarmEvent::Behaviour(gossip_event) => {
                        self.handle_gossipsub_event(gossip_event);
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_gossipsub_event(&mut self, event: gossipsub::Event) {
        match event {
            gossipsub::Event::Message {
                propagation_source,
                message_id: _,
                message,
            } => {
                info!("📩 Received gossipsub message from {}", propagation_source);

                // Try to decode as Block
                if let Ok(block) = bincode::deserialize::<Block>(&message.data) {
                    info!("   📦 Received block: height {}", block.header.block_height);
                    info!("   Hash: {}", hex::encode(block.hash().as_bytes()));
                    // TODO: Validate and add to storage
                }
                // Try to decode as Transaction
                else if let Ok(tx) = bincode::deserialize::<Transaction>(&message.data) {
                    info!("   💸 Received transaction: {} → {}", tx.from, tx.to);
                    // TODO: Add to mempool
                } else {
                    warn!("   ⚠️ Unknown message type (size: {})", message.data.len());
                }
            }
            gossipsub::Event::Subscribed { peer_id, topic } => {
                info!("👤 Peer {} subscribed to {}", peer_id, topic);
            }
            gossipsub::Event::Unsubscribed { peer_id, topic } => {
                info!("👋 Peer {} unsubscribed from {}", peer_id, topic);
            }
            gossipsub::Event::GossipsubNotSupported { peer_id } => {
                warn!("⚠️ Peer {} doesn't support Gossipsub", peer_id);
            }
        }
    }

    pub async fn broadcast_block(&mut self, block: &Block) -> Result<()> {
        let data = bincode::serialize(block)
            .map_err(|e| SpiraChainError::NetworkError(format!("Serialize block: {}", e)))?;

        self.swarm
            .behaviour_mut()
            .publish(self.block_topic.clone(), data)
            .map_err(|e| SpiraChainError::NetworkError(format!("Broadcast block: {}", e)))?;

        info!(
            "📡 Broadcasted block {} to {} peers",
            block.header.block_height,
            self.connected_peers.len()
        );
        Ok(())
    }

    pub async fn broadcast_transaction(&mut self, tx: &Transaction) -> Result<()> {
        let data = bincode::serialize(tx)
            .map_err(|e| SpiraChainError::NetworkError(format!("Serialize tx: {}", e)))?;

        self.swarm
            .behaviour_mut()
            .publish(self.tx_topic.clone(), data)
            .map_err(|e| SpiraChainError::NetworkError(format!("Broadcast tx: {}", e)))?;

        info!("📡 Broadcasted transaction to network");
        Ok(())
    }

    pub fn get_peer_count(&self) -> usize {
        self.connected_peers.len()
    }

    /// Request blocks from a peer (via Gossipsub for now)
    pub fn request_blocks(&mut self, _peer_id: PeerId, start_height: u64, count: u64) -> Result<()> {
        debug!("📥 Block sync: {}-{} (via Gossipsub propagation)", start_height, start_height + count - 1);
        Ok(())
    }

    /// Get list of connected peer IDs
    pub fn get_connected_peers(&self) -> Vec<PeerId> {
        self.connected_peers.iter().copied().collect()
    }

    pub fn get_peer_id(&self) -> PeerId {
        self.local_peer_id
    }

    /// Trigger peer discovery (placeholder)
    pub fn discover_more_peers(&mut self) {
        debug!("🔄 Peer discovery via Gossipsub");
    }

    /// Get routing table stats (placeholder)
    pub fn get_routing_table_size(&mut self) -> usize {
        self.connected_peers.len()
    }
}
