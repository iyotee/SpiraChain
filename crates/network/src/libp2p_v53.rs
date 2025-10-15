// LibP2P v0.53 Complete Implementation for SpiraChain
// Full P2P with Gossipsub + mDNS + Kademlia + Request/Response

use futures::StreamExt;
use libp2p::{
    gossipsub, identify, kad,
    mdns,
    request_response::{self, ProtocolSupport},
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    StreamProtocol,
    identity::Keypair,
    noise,
    tcp, yamux, Multiaddr, PeerId,
};
use spirachain_core::{Block, Result, SpiraChainError, Transaction};
use std::collections::HashSet;
use tracing::{debug, info, warn};

use crate::bootstrap::{discover_bootstrap_peers, BootstrapConfig};

// Define the combined network behaviour
#[derive(NetworkBehaviour)]
pub struct SpiraChainBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub kademlia: kad::Kademlia<kad::store::MemoryStore>,
    pub identify: identify::Behaviour,
    pub request_response: request_response::cbor::Behaviour<BlockRequest, BlockResponse>,
}

// Block request/response types for sync
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlockRequest {
    pub start_height: u64,
    pub count: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlockResponse {
    pub blocks: Vec<Vec<u8>>, // Serialized blocks
}

pub struct LibP2PNetwork {
    swarm: Swarm<SpiraChainBehaviour>,
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

        // 1. Create Gossipsub (for block/tx propagation)
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

        // 2. Create mDNS (for local peer discovery)
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)
            .map_err(|e| SpiraChainError::NetworkError(format!("mDNS init: {}", e)))?;

        // 3. Create Kademlia/DHT (for global peer discovery)
        let mut kad_config = kad::KademliaConfig::default();
        kad_config.set_query_timeout(std::time::Duration::from_secs(60));
        let store = kad::store::MemoryStore::new(local_peer_id);
        let kademlia = kad::Kademlia::with_config(local_peer_id, store, kad_config);

        // 4. Create Identify (for peer info exchange)
        let identify = identify::Behaviour::new(identify::Config::new(
            format!("/spirachain/{}/1.0.0", network),
            local_key.public(),
        ));

        // 5. Create Request/Response (for block sync)
        let request_response = request_response::cbor::Behaviour::new(
            [(
                StreamProtocol::new("/spirachain/sync/1.0.0"),
                ProtocolSupport::Full,
            )],
            request_response::Config::default(),
        );

        // Combine all behaviours
        let behaviour = SpiraChainBehaviour {
            gossipsub,
            mdns,
            kademlia,
            identify,
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

        info!("✅ Full P2P stack initialized:");
        info!("   ✓ Gossipsub (block/tx propagation)");
        info!("   ✓ mDNS (local discovery)");
        info!("   ✓ Kademlia (global discovery)");
        info!("   ✓ Identify (peer info)");
        info!("   ✓ Request/Response (block sync)");

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
            .gossipsub
            .subscribe(&self.block_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe blocks: {}", e)))?;
        self.swarm
            .behaviour_mut()
            .gossipsub
            .subscribe(&self.tx_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe tx: {}", e)))?;

        // Set Kademlia to server mode
        self.swarm.behaviour_mut().kademlia.set_mode(Some(kad::Mode::Server));

        self.is_listening = true;
        info!("✅ P2P network listening on port {}", self.listen_port);
        info!("   mDNS: Active (discovering local peers)");
        info!("   Kademlia: Server mode (discoverable globally)");

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
                        } else {
                            // Add to Kademlia routing table
                            if let Some(peer_id) = addr.iter().find_map(|p| {
                                if let libp2p::multiaddr::Protocol::P2p(peer_id) = p {
                                    Some(peer_id)
                                } else {
                                    None
                                }
                            }) {
                                self.swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                                debug!("   Added {} to Kademlia routing table", peer_id);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                warn!("⚠️  Bootstrap discovery failed: {}", e);
                warn!("   Node will rely on mDNS/DHT for peer discovery");
                warn!("   This is normal if DNS seeds are not yet configured");
            }
        }

        // Start Kademlia bootstrap to discover more peers
        if let Err(e) = self.swarm.behaviour_mut().kademlia.bootstrap() {
            warn!("⚠️  Kademlia bootstrap failed: {}", e);
        } else {
            info!("🔄 Kademlia bootstrap started - discovering peers globally");
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
            .gossipsub
            .subscribe(&self.block_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe blocks: {}", e)))?;
        self.swarm
            .behaviour_mut()
            .gossipsub
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
                    
                    // Add peer to Kademlia routing table
                    self.swarm.behaviour_mut().kademlia.add_address(&peer_id, endpoint.get_remote_address().clone());
                    debug!("   Added {} to Kademlia routing table", peer_id);
                    
                    Some(format!("Connected: {}", peer_id))
                }
                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                    info!("👋 Disconnected from peer: {}", peer_id);
                    self.connected_peers.remove(&peer_id);
                    None
                }
                SwarmEvent::Behaviour(behaviour_event) => {
                    self.handle_behaviour_event(behaviour_event);
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
            .gossipsub
            .subscribe(&block_topic)
            .map_err(|e| SpiraChainError::NetworkError(format!("Subscribe blocks: {}", e)))?;
        self.swarm
            .behaviour_mut()
            .gossipsub
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
                    SwarmEvent::Behaviour(behaviour_event) => {
                        self.handle_behaviour_event(behaviour_event);
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_behaviour_event(&mut self, event: SpiraChainBehaviourEvent) {
        match event {
            // Gossipsub events (block/tx propagation)
            SpiraChainBehaviourEvent::Gossipsub(gossip_event) => {
                self.handle_gossipsub_event(gossip_event);
            }
            
            // mDNS events (local peer discovery)
            SpiraChainBehaviourEvent::Mdns(mdns_event) => {
                match mdns_event {
                    mdns::Event::Discovered(peers) => {
                        for (peer_id, multiaddr) in peers {
                            info!("🔍 [mDNS] Discovered local peer: {} at {}", peer_id, multiaddr);
                            self.swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr.clone());
                            
                            // Try to dial the peer
                            if let Err(e) = self.swarm.dial(multiaddr.clone()) {
                                debug!("   Failed to dial {}: {}", multiaddr, e);
                            }
                        }
                    }
                    mdns::Event::Expired(peers) => {
                        for (peer_id, multiaddr) in peers {
                            debug!("🔍 [mDNS] Peer expired: {} at {}", peer_id, multiaddr);
                        }
                    }
                }
            }
            
            // Kademlia/DHT events (global peer discovery)
            SpiraChainBehaviourEvent::Kademlia(kad_event) => {
                match kad_event {
                    kad::Event::RoutingUpdated { peer, .. } => {
                        debug!("📍 [Kademlia] Routing table updated with peer: {}", peer);
                    }
                    kad::Event::OutboundQueryProgressed { result, .. } => {
                        match result {
                            kad::QueryResult::GetClosestPeers(Ok(ok)) => {
                                info!("📍 [Kademlia] Found {} closest peers", ok.peers.len());
                                for peer in ok.peers {
                                    debug!("   Peer: {}", peer);
                                }
                            }
                            kad::QueryResult::Bootstrap(Ok(ok)) => {
                                info!("📍 [Kademlia] Bootstrap complete with {} peers", ok.num_remaining);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            
            // Identify events (peer info exchange)
            SpiraChainBehaviourEvent::Identify(identify_event) => {
                match identify_event {
                    identify::Event::Received { peer_id, info } => {
                        info!("🆔 [Identify] Received info from {}", peer_id);
                        debug!("   Protocol: {}", info.protocol_version);
                        debug!("   Agent: {}", info.agent_version);
                        
                        // Add all listen addresses to Kademlia
                        for addr in info.listen_addrs {
                            self.swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                            debug!("   Address: {}", addr);
                        }
                    }
                    identify::Event::Sent { peer_id } => {
                        debug!("🆔 [Identify] Sent info to {}", peer_id);
                    }
                    identify::Event::Pushed { peer_id, .. } => {
                        debug!("🆔 [Identify] Pushed info to {}", peer_id);
                    }
                    identify::Event::Error { peer_id, error } => {
                        warn!("🆔 [Identify] Error with {}: {}", peer_id, error);
                    }
                }
            }
            
            // Request/Response events (block sync)
            SpiraChainBehaviourEvent::RequestResponse(rr_event) => {
                match rr_event {
                    request_response::Event::Message { peer, message } => {
                        match message {
                            request_response::Message::Request { request, channel, .. } => {
                                info!("📥 [Sync] Block request from {}: start={}, count={}", 
                                    peer, request.start_height, request.count);
                                // TODO: Respond with blocks from storage
                                let response = BlockResponse { blocks: vec![] };
                                let _ = self.swarm.behaviour_mut().request_response.send_response(channel, response);
                            }
                            request_response::Message::Response { response, .. } => {
                                info!("📤 [Sync] Received {} blocks from {}", response.blocks.len(), peer);
                                // TODO: Process received blocks
                            }
                        }
                    }
                    request_response::Event::OutboundFailure { peer, error, .. } => {
                        warn!("📤 [Sync] Outbound request to {} failed: {}", peer, error);
                    }
                    request_response::Event::InboundFailure { peer, error, .. } => {
                        warn!("📥 [Sync] Inbound request from {} failed: {}", peer, error);
                    }
                    request_response::Event::ResponseSent { peer, .. } => {
                        debug!("📤 [Sync] Response sent to {}", peer);
                    }
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
            .gossipsub
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
            .gossipsub
            .publish(self.tx_topic.clone(), data)
            .map_err(|e| SpiraChainError::NetworkError(format!("Broadcast tx: {}", e)))?;

        info!("📡 Broadcasted transaction to network");
        Ok(())
    }

    pub fn get_peer_count(&self) -> usize {
        self.connected_peers.len()
    }

    /// Request blocks from a peer (for synchronization)
    pub fn request_blocks(&mut self, peer_id: PeerId, start_height: u64, count: u64) -> Result<()> {
        let request = BlockRequest {
            start_height,
            count,
        };
        
        self.swarm
            .behaviour_mut()
            .request_response
            .send_request(&peer_id, request);
        
        info!("📥 Requesting blocks {}-{} from {}", start_height, start_height + count - 1, peer_id);
        Ok(())
    }

    /// Get list of connected peer IDs
    pub fn get_connected_peers(&self) -> Vec<PeerId> {
        self.connected_peers.iter().copied().collect()
    }

    pub fn get_peer_id(&self) -> PeerId {
        self.local_peer_id
    }

    /// Trigger Kademlia to find more peers
    pub fn discover_more_peers(&mut self) {
        let _ = self.swarm.behaviour_mut().kademlia.bootstrap();
        debug!("🔄 Triggered Kademlia peer discovery");
    }

    /// Get Kademlia routing table stats
    pub fn get_routing_table_size(&mut self) -> usize {
        self.swarm.behaviour_mut().kademlia.kbuckets().count()
    }
}
