use libp2p::{
    gossipsub::{self, MessageId, GossipsubEvent, MessageAuthenticity, ValidationMode, GossipsubConfigBuilder, IdentTopic},
    mdns::{self, tokio::Behaviour as MdnsBehaviour, MdnsEvent},
    kad::{self, KademliaEvent, store::MemoryStore, KademliaConfig},
    swarm::{NetworkBehaviour, SwarmEvent, Swarm, SwarmBuilder},
    identity, PeerId, Multiaddr, Transport, core::upgrade,
    tcp::TcpConfig, noise::NoiseConfig, yamux::YamuxConfig,
};
use spirachain_core::{Block, Transaction, Result, SpiraChainError};
use crate::protocol::NetworkMessage;
use tokio::sync::mpsc;
use tracing::{info, warn, error, debug};
use std::collections::{HashSet, HashMap};
use std::time::Duration;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(NetworkBehaviour)]
pub struct SpiraChainBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: MdnsBehaviour,
    pub kademlia: kad::Behaviour<MemoryStore>,
}

#[derive(Debug)]
pub enum SpiraChainEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
    Kademlia(KademliaEvent),
    Message(NetworkMessage),
}

impl From<GossipsubEvent> for SpiraChainEvent {
    fn from(event: GossipsubEvent) -> Self {
        SpiraChainEvent::Gossipsub(event)
    }
}

impl From<MdnsEvent> for SpiraChainEvent {
    fn from(event: MdnsEvent) -> Self {
        SpiraChainEvent::Mdns(event)
    }
}

impl From<KademliaEvent> for SpiraChainEvent {
    fn from(event: KademliaEvent) -> Self {
        SpiraChainEvent::Kademlia(event)
    }
}

pub struct LibP2PNetwork {
    swarm: Swarm<SpiraChainBehaviour>,
    local_peer_id: PeerId,
    connected_peers: HashSet<PeerId>,
    message_tx: mpsc::UnboundedSender<NetworkMessage>,
    message_rx: mpsc::UnboundedReceiver<NetworkMessage>,
    block_topic: IdentTopic,
    tx_topic: IdentTopic,
    peer_addresses: HashMap<PeerId, Multiaddr>,
}

impl LibP2PNetwork {
    pub async fn new(port: u16) -> Result<Self> {
        info!("🌐 Initializing full LibP2P network");
        info!("   Port: {}", port);

        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        info!("   Peer ID: {}", local_peer_id);

        let (message_tx, message_rx) = mpsc::unbounded_channel();

        let block_topic = IdentTopic::new("spirachain-blocks");
        let tx_topic = IdentTopic::new("spirachain-transactions");

        // Create transport
        let transport = TcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(&local_key).unwrap())
            .multiplex(YamuxConfig::default())
            .boxed();

        // Create message ID function for gossipsub
        let message_id_fn = |message: &gossipsub::Message| {
            let mut hasher = DefaultHasher::new();
            message.data.hash(&mut hasher);
            MessageId::from(hasher.finish().to_string())
        };

        // Configure gossipsub
        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            .build()
            .map_err(|e| SpiraChainError::NetworkError(format!("Gossipsub config error: {}", e)))?;

        let gossipsub = gossipsub::Behaviour::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        ).map_err(|e| SpiraChainError::NetworkError(format!("Gossipsub init error: {}", e)))?;

        // Configure mDNS
        let mdns = MdnsBehaviour::new(mdns::Config::default())
            .await
            .map_err(|e| SpiraChainError::NetworkError(format!("mDNS init error: {}", e)))?;

        // Configure Kademlia
        let mut kademlia_config = KademliaConfig::default();
        kademlia_config.set_query_timeout(Duration::from_secs(60));
        let mut kademlia = kad::Behaviour::new(local_peer_id, MemoryStore::new(local_peer_id), kademlia_config);

        // Create behaviour
        let behaviour = SpiraChainBehaviour {
            gossipsub,
            mdns,
            kademlia,
        };

        // Create swarm
        let swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, local_peer_id)
            .build();

        info!("✅ LibP2P network configured");
        info!("   Topics: blocks, transactions");
        info!("   Discovery: mDNS (local) + Kademlia DHT (global)");

        Ok(Self {
            swarm,
            local_peer_id,
            connected_peers: HashSet::new(),
            message_tx,
            message_rx,
            block_topic,
            tx_topic,
            peer_addresses: HashMap::new(),
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 Starting LibP2P network");

        // Listen on all interfaces
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
            .map_err(|e| SpiraChainError::NetworkError(format!("Failed to listen: {}", e)))?;

        // Subscribe to topics
        self.swarm.behaviour_mut().gossipsub.subscribe(&self.block_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Failed to subscribe to blocks: {}", e)))?;
        self.swarm.behaviour_mut().gossipsub.subscribe(&self.tx_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Failed to subscribe to transactions: {}", e)))?;

        info!("✅ LibP2P network started");
        info!("   Listening on all interfaces");
        info!("   Subscribed to: blocks, transactions");

        // Main event loop
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            info!("📡 Listening on: {}", address);
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                            info!("🤝 Connected to peer: {}", peer_id);
                            self.connected_peers.insert(peer_id);
                            
                            // Add to Kademlia
                            if let Some(addr) = endpoint.get_remote_address() {
                                self.peer_addresses.insert(peer_id, addr.clone());
                                self.swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                            }
                        }
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            info!("👋 Disconnected from peer: {}", peer_id);
                            self.connected_peers.remove(&peer_id);
                            self.peer_addresses.remove(&peer_id);
                        }
                        SwarmEvent::Behaviour(event) => {
                            self.handle_behaviour_event(event).await?;
                        }
                        _ => {}
                    }
                }
                message = self.message_rx.recv() => {
                    if let Some(msg) = message {
                        self.handle_outgoing_message(msg).await?;
                    }
                }
            }
        }
    }

    async fn handle_behaviour_event(&mut self, event: SpiraChainEvent) -> Result<()> {
        match event {
            SpiraChainEvent::Gossipsub(gossipsub_event) => {
                match gossipsub_event {
                    GossipsubEvent::Message { propagation_source: _peer_id, message_id: _id, message } => {
                        debug!("📨 Received gossipsub message");
                        
                        // Deserialize and forward
                        if let Ok(network_msg) = bincode::deserialize::<NetworkMessage>(&message.data) {
                            self.handle_network_message(network_msg).await?;
                        }
                    }
                    GossipsubEvent::Subscribed { topic, peer_id } => {
                        info!("📋 Peer {} subscribed to topic: {}", peer_id, topic);
                    }
                    GossipsubEvent::Unsubscribed { topic, peer_id } => {
                        info!("📋 Peer {} unsubscribed from topic: {}", peer_id, topic);
                    }
                    _ => {
                        debug!("Gossipsub event: {:?}", gossipsub_event);
                    }
                }
            }
            SpiraChainEvent::Mdns(mdns_event) => {
                match mdns_event {
                    MdnsEvent::Discovered(list) => {
                        for (peer_id, multiaddr) in list {
                            info!("🔍 mDNS discovered peer: {} at {}", peer_id, multiaddr);
                            
                            // Add to Kademlia and connect
                            self.peer_addresses.insert(peer_id, multiaddr.clone());
                            self.swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr.clone());
                            
                            if let Err(e) = self.swarm.dial(multiaddr) {
                                warn!("Failed to dial discovered peer {}: {}", peer_id, e);
                            }
                        }
                    }
                    MdnsEvent::Expired(list) => {
                        for (peer_id, multiaddr) in list {
                            info!("⏰ mDNS expired peer: {} at {}", peer_id, multiaddr);
                            self.peer_addresses.remove(&peer_id);
                        }
                    }
                }
            }
            SpiraChainEvent::Kademlia(kademlia_event) => {
                match kademlia_event {
                    KademliaEvent::RoutingUpdated { peer, .. } => {
                        debug!("🔄 Kademlia routing updated for peer: {}", peer);
                    }
                    KademliaEvent::RoutablePeer { peer, address } => {
                        info!("🌐 Kademlia found routable peer: {} at {}", peer, address);
                        self.peer_addresses.insert(peer, address);
                        
                        // Try to connect
                        if let Err(e) = self.swarm.dial(address) {
                            debug!("Failed to dial Kademlia peer: {}", e);
                        }
                    }
                    _ => {
                        debug!("Kademlia event: {:?}", kademlia_event);
                    }
                }
            }
            SpiraChainEvent::Message(msg) => {
                debug!("📨 Received direct message: {:?}", msg);
            }
        }
        Ok(())
    }

    async fn handle_network_message(&mut self, msg: NetworkMessage) -> Result<()> {
        match msg {
            NetworkMessage::NewBlock { block } => {
                info!("📦 Received new block: {}", block.header.height);
                // TODO: Validate and process block
            }
            NetworkMessage::NewTransaction { transaction } => {
                debug!("💰 Received new transaction: {}", transaction.hash());
                // TODO: Validate and process transaction
            }
            NetworkMessage::BlockRequest { block_hash } => {
                debug!("📥 Block request for: {}", block_hash);
                // TODO: Send block if we have it
            }
            NetworkMessage::BlockResponse { block } => {
                info!("📤 Received block response: {}", block.header.height);
                // TODO: Process block response
            }
            NetworkMessage::TransactionRequest { tx_hash } => {
                debug!("📥 Transaction request for: {}", tx_hash);
                // TODO: Send transaction if we have it
            }
            NetworkMessage::TransactionResponse { transaction } => {
                debug!("📤 Received transaction response: {}", transaction.hash());
                // TODO: Process transaction response
            }
            NetworkMessage::PeerInfo { peer_id, addresses } => {
                info!("👤 Peer info for {}: {} addresses", peer_id, addresses.len());
                for addr in addresses {
                    self.peer_addresses.insert(peer_id, addr);
                }
            }
            NetworkMessage::Ping { nonce } => {
                debug!("🏓 Received ping: {}", nonce);
                // TODO: Send pong
            }
            NetworkMessage::Pong { nonce } => {
                debug!("🏓 Received pong: {}", nonce);
                // TODO: Update latency
            }
            NetworkMessage::EncryptedMessage { peer_id, data } => {
                debug!("🔒 Received encrypted message from {}: {} bytes", peer_id, data.len());
                // TODO: Decrypt and process
            }
        }
        Ok(())
    }

    async fn handle_outgoing_message(&mut self, msg: NetworkMessage) -> Result<()> {
        match msg {
            NetworkMessage::NewBlock { .. } | NetworkMessage::NewTransaction { .. } => {
                // Broadcast via gossipsub
                let topic = if matches!(msg, NetworkMessage::NewBlock { .. }) {
                    &self.block_topic
                } else {
                    &self.tx_topic
                };

                let data = bincode::serialize(&msg)
                    .map_err(|e| SpiraChainError::NetworkError(format!("Serialization error: {}", e)))?;

                self.swarm.behaviour_mut().gossipsub.publish(topic.clone(), data)
                    .map_err(|e| SpiraChainError::NetworkError(format!("Publish error: {}", e)))?;

                info!("📤 Broadcasted message via gossipsub");
            }
            _ => {
                // For other messages, we'd need direct peer communication
                debug!("📤 Outgoing message (not broadcast): {:?}", msg);
            }
        }
        Ok(())
    }

    pub fn broadcast_block(&self, block: &Block) -> Result<()> {
        let msg = NetworkMessage::NewBlock { block: block.clone() };
        self.message_tx.send(msg)
            .map_err(|_| SpiraChainError::NetworkError("Failed to send block".to_string()))?;
        Ok(())
    }

    pub fn broadcast_transaction(&self, transaction: &Transaction) -> Result<()> {
        let msg = NetworkMessage::NewTransaction { transaction: transaction.clone() };
        self.message_tx.send(msg)
            .map_err(|_| SpiraChainError::NetworkError("Failed to send transaction".to_string()))?;
        Ok(())
    }

    pub fn get_peer_count(&self) -> usize {
        self.connected_peers.len()
    }

    pub fn get_peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }

    pub fn get_connected_peers(&self) -> &HashSet<PeerId> {
        &self.connected_peers
    }

    pub fn print_network_stats(&self) {
        info!("📊 Network Stats:");
        info!("   Peer ID: {}", self.local_peer_id);
        info!("   Connected peers: {}", self.connected_peers.len());
        info!("   Known addresses: {}", self.peer_addresses.len());
        
        if !self.connected_peers.is_empty() {
            info!("   Peers: {:?}", self.connected_peers);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_libp2p_network_creation() {
        let network = LibP2PNetwork::new(0).await;
        assert!(network.is_ok());
        
        let network = network.unwrap();
        assert_eq!(network.get_peer_count(), 0);
        assert!(network.get_connected_peers().is_empty());
    }

    #[tokio::test]
    async fn test_network_stats() {
        let network = LibP2PNetwork::new(0).await.unwrap();
        
        // Should not panic
        network.print_network_stats();
        
        assert_eq!(network.get_peer_count(), 0);
    }

    #[tokio::test]
    async fn test_peer_discovery() {
        // Test that we can create two networks
        let network1 = LibP2PNetwork::new(0).await.unwrap();
        let network2 = LibP2PNetwork::new(0).await.unwrap();
        
        // They should have different peer IDs
        assert_ne!(network1.get_peer_id(), network2.get_peer_id());
        
        // Both should start with 0 peers
        assert_eq!(network1.get_peer_count(), 0);
        assert_eq!(network2.get_peer_count(), 0);
    }

    #[tokio::test]
    async fn test_broadcast_interface() {
        let network = LibP2PNetwork::new(0).await.unwrap();
        
        // Create a dummy block and transaction
        let block = Block::new(
            spirachain_core::BlockHeader::new(1, [0u8; 32], [0u8; 32], 0),
            vec![],
        );
        
        let transaction = Transaction::new(
            [0u8; 32],
            [0u8; 32],
            100,
            0,
        );
        
        // Should not panic (though won't actually broadcast without peers)
        network.broadcast_block(&block).unwrap();
        network.broadcast_transaction(&transaction).unwrap();
    }
}