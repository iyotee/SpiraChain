// LibP2P v0.53 Simple Implementation
// Simplified P2P for SpiraChain using modern libp2p API

use libp2p::{
    gossipsub, mdns, noise, tcp, yamux,
    identity::Keypair,
    swarm::{NetworkBehaviour, SwarmEvent, Swarm},
    PeerId,
};
use spirachain_core::{Block, Transaction, Result, SpiraChainError};
use tokio::sync::mpsc;
use tracing::{info, warn, error};
use std::collections::HashSet;

#[derive(NetworkBehaviour)]
pub struct SpiraChainBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub struct LibP2PNetwork {
    swarm: Swarm<SpiraChainBehaviour>,
    local_peer_id: PeerId,
    connected_peers: HashSet<PeerId>,
}

impl LibP2PNetwork {
    pub async fn new(port: u16) -> Result<Self> {
        info!("🌐 Initializing LibP2P Network (v0.53)");
        
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
        
        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        ).map_err(|e| SpiraChainError::NetworkError(format!("Gossipsub init: {}", e)))?;
        
        // Create mDNS
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)
            .map_err(|e| SpiraChainError::NetworkError(format!("mDNS init: {}", e)))?;
        
        // Combine into behaviour
        let behaviour = SpiraChainBehaviour {
            gossipsub,
            mdns,
        };
        
        // Create Swarm
        let swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|e| SpiraChainError::NetworkError(format!("TCP transport: {}", e)))?
            .with_behaviour(|_key| behaviour)
            .map_err(|e| SpiraChainError::NetworkError(format!("Behaviour: {}", e)))?
            .build();
        
        Ok(Self {
            swarm,
            local_peer_id,
            connected_peers: HashSet::new(),
        })
    }
    
    pub async fn start(&mut self) -> Result<()> {
        // Listen on all interfaces
        let listen_addr = format!("/ip4/0.0.0.0/tcp/0")
            .parse()
            .map_err(|e| SpiraChainError::NetworkError(format!("Invalid addr: {}", e)))?;
        
        self.swarm.listen_on(listen_addr)
            .map_err(|e| SpiraChainError::NetworkError(format!("Listen failed: {}", e)))?;
        
        // Subscribe to topics
        let block_topic = gossipsub::IdentTopic::new("spirachain-blocks");
        let tx_topic = gossipsub::IdentTopic::new("spirachain-transactions");
        
        self.swarm.behaviour_mut().gossipsub.subscribe(&block_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe blocks: {}", e)))?;
        self.swarm.behaviour_mut().gossipsub.subscribe(&tx_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe tx: {}", e)))?;
        
        info!("✅ P2P network listening");
        info!("   Topics: spirachain-blocks, spirachain-transactions");
        
        // Event loop
        use futures::StreamExt;
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
                SwarmEvent::Behaviour(event) => {
                    self.handle_behaviour_event(event);
                }
                _ => {}
            }
            }
        }
    }
    
    fn handle_behaviour_event(&mut self, event: <SpiraChainBehaviour as NetworkBehaviour>::ToSwarm) {
        match event {
            SpiraChainBehaviourEvent::Gossipsub(gossip_event) => {
                match gossip_event {
                    gossipsub::Event::Message { propagation_source, message_id, message } => {
                        info!("📩 Received message from {}", propagation_source);
                        
                        // Try to decode as Block
                        if let Ok(block) = bincode::deserialize::<Block>(&message.data) {
                            info!("   📦 Received block: height {}", block.header.block_height);
                            // TODO: Validate and add to storage
                        }
                        // Try to decode as Transaction
                        else if let Ok(tx) = bincode::deserialize::<Transaction>(&message.data) {
                            info!("   💸 Received transaction: {} → {}", tx.from, tx.to);
                            // TODO: Add to mempool
                        }
                    }
                    gossipsub::Event::Subscribed { peer_id, topic } => {
                        info!("👤 Peer {} subscribed to {}", peer_id, topic);
                    }
                    _ => {}
                }
            }
            SpiraChainBehaviourEvent::Mdns(mdns_event) => {
                match mdns_event {
                    mdns::Event::Discovered(peers) => {
                        for (peer_id, addr) in peers {
                            info!("🔍 Discovered peer: {} at {}", peer_id, addr);
                            // Dial the peer
                            if let Err(e) = self.swarm.dial(addr) {
                                warn!("Failed to dial peer {}: {}", peer_id, e);
                            }
                        }
                    }
                    mdns::Event::Expired(peers) => {
                        for (peer_id, _addr) in peers {
                            info!("⏰ Peer expired: {}", peer_id);
                        }
                    }
                }
            }
        }
    }
    
    pub async fn broadcast_block(&mut self, block: &Block) -> Result<()> {
        let topic = gossipsub::IdentTopic::new("spirachain-blocks");
        let data = bincode::serialize(block)
            .map_err(|e| SpiraChainError::NetworkError(format!("Serialize block: {}", e)))?;
        
        self.swarm.behaviour_mut().gossipsub
            .publish(topic, data)
            .map_err(|e| SpiraChainError::NetworkError(format!("Broadcast block: {}", e)))?;
        
        Ok(())
    }
    
    pub async fn broadcast_transaction(&mut self, tx: &Transaction) -> Result<()> {
        let topic = gossipsub::IdentTopic::new("spirachain-transactions");
        let data = bincode::serialize(tx)
            .map_err(|e| SpiraChainError::NetworkError(format!("Serialize tx: {}", e)))?;
        
        self.swarm.behaviour_mut().gossipsub
            .publish(topic, data)
            .map_err(|e| SpiraChainError::NetworkError(format!("Broadcast tx: {}", e)))?;
        
        Ok(())
    }
    
    pub fn get_peer_count(&self) -> usize {
        self.connected_peers.len()
    }
    
    pub fn get_peer_id(&self) -> PeerId {
        self.local_peer_id
    }
}

