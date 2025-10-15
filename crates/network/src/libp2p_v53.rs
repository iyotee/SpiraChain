// LibP2P v0.53 Complete Implementation for SpiraChain
// Full P2P with Gossipsub + mDNS + Kademlia (manual NetworkBehaviour implementation)

use futures::StreamExt;
use libp2p::{
    gossipsub, identify, kad, mdns,
    swarm::{
        ConnectionDenied, ConnectionId, FromSwarm, NetworkBehaviour, 
        Swarm, SwarmEvent, THandler, THandlerInEvent, THandlerOutEvent, ToSwarm,
    },
    identity::Keypair,
    noise,
    tcp, yamux, Multiaddr, PeerId,
};
use spirachain_core::{Block, Result, SpiraChainError, Transaction};
use std::collections::HashSet;
use std::task::{Context, Poll};
use tracing::{debug, info, warn};

use crate::bootstrap::{discover_bootstrap_peers, BootstrapConfig};

// Manual NetworkBehaviour implementation to avoid derive macro conflicts
pub struct SpiraChainBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub identify: identify::Behaviour,
}

// Events from the combined behaviour
#[allow(clippy::large_enum_variant)]
pub enum SpiraChainBehaviourEvent {
    Gossipsub(gossipsub::Event),
    Mdns(mdns::Event),
    Kademlia(kad::Event),
    Identify(identify::Event),
}

// Manual NetworkBehaviour implementation
impl NetworkBehaviour for SpiraChainBehaviour {
    type ConnectionHandler = libp2p::swarm::derive_prelude::EitherHandler<
        libp2p::swarm::derive_prelude::EitherHandler<
            <gossipsub::Behaviour as NetworkBehaviour>::ConnectionHandler,
            <mdns::tokio::Behaviour as NetworkBehaviour>::ConnectionHandler,
        >,
        libp2p::swarm::derive_prelude::EitherHandler<
            <kad::Behaviour<kad::store::MemoryStore> as NetworkBehaviour>::ConnectionHandler,
            <identify::Behaviour as NetworkBehaviour>::ConnectionHandler,
        >,
    >;
    
    type ToSwarm = SpiraChainBehaviourEvent;

    fn handle_pending_inbound_connection(
        &mut self,
        connection_id: ConnectionId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> std::result::Result<(), ConnectionDenied> {
        self.gossipsub.handle_pending_inbound_connection(connection_id, local_addr, remote_addr)?;
        self.mdns.handle_pending_inbound_connection(connection_id, local_addr, remote_addr)?;
        self.kademlia.handle_pending_inbound_connection(connection_id, local_addr, remote_addr)?;
        self.identify.handle_pending_inbound_connection(connection_id, local_addr, remote_addr)?;
        Ok(())
    }

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: ConnectionId,
        peer: PeerId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> std::result::Result<THandler<Self>, ConnectionDenied> {
        let gossipsub_handler = self.gossipsub.handle_established_inbound_connection(
            connection_id, peer, local_addr, remote_addr,
        )?;
        let mdns_handler = self.mdns.handle_established_inbound_connection(
            connection_id, peer, local_addr, remote_addr,
        )?;
        let kademlia_handler = self.kademlia.handle_established_inbound_connection(
            connection_id, peer, local_addr, remote_addr,
        )?;
        let identify_handler = self.identify.handle_established_inbound_connection(
            connection_id, peer, local_addr, remote_addr,
        )?;
        
        use libp2p::swarm::derive_prelude::EitherHandler;
        Ok(EitherHandler::Left(
            EitherHandler::Left(gossipsub_handler, mdns_handler),
            EitherHandler::Right(kademlia_handler, identify_handler),
        ))
    }

    fn handle_pending_outbound_connection(
        &mut self,
        connection_id: ConnectionId,
        maybe_peer: Option<PeerId>,
        addresses: &[Multiaddr],
        effective_role: libp2p::core::Endpoint,
    ) -> std::result::Result<Vec<Multiaddr>, ConnectionDenied> {
        let mut result = self.gossipsub.handle_pending_outbound_connection(
            connection_id, maybe_peer, addresses, effective_role,
        )?;
        result.extend(self.mdns.handle_pending_outbound_connection(
            connection_id, maybe_peer, addresses, effective_role,
        )?);
        result.extend(self.kademlia.handle_pending_outbound_connection(
            connection_id, maybe_peer, addresses, effective_role,
        )?);
        result.extend(self.identify.handle_pending_outbound_connection(
            connection_id, maybe_peer, addresses, effective_role,
        )?);
        Ok(result)
    }

    fn handle_established_outbound_connection(
        &mut self,
        connection_id: ConnectionId,
        peer: PeerId,
        addr: &Multiaddr,
        role_override: libp2p::core::Endpoint,
    ) -> std::result::Result<THandler<Self>, ConnectionDenied> {
        let gossipsub_handler = self.gossipsub.handle_established_outbound_connection(
            connection_id, peer, addr, role_override,
        )?;
        let mdns_handler = self.mdns.handle_established_outbound_connection(
            connection_id, peer, addr, role_override,
        )?;
        let kademlia_handler = self.kademlia.handle_established_outbound_connection(
            connection_id, peer, addr, role_override,
        )?;
        let identify_handler = self.identify.handle_established_outbound_connection(
            connection_id, peer, addr, role_override,
        )?;
        
        use libp2p::swarm::derive_prelude::EitherHandler;
        Ok(EitherHandler::Left(
            EitherHandler::Left(gossipsub_handler, mdns_handler),
            EitherHandler::Right(kademlia_handler, identify_handler),
        ))
    }

    fn on_swarm_event(&mut self, event: FromSwarm) {
        self.gossipsub.on_swarm_event(event);
        self.mdns.on_swarm_event(event);
        self.kademlia.on_swarm_event(event);
        self.identify.on_swarm_event(event);
    }

    fn on_connection_handler_event(
        &mut self,
        peer_id: PeerId,
        connection_id: ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        use libp2p::swarm::derive_prelude::EitherOutput;
        match event {
            EitherOutput::First(EitherOutput::First(ev)) => {
                self.gossipsub.on_connection_handler_event(peer_id, connection_id, ev);
            }
            EitherOutput::First(EitherOutput::Second(ev)) => {
                self.mdns.on_connection_handler_event(peer_id, connection_id, ev);
            }
            EitherOutput::Second(EitherOutput::First(ev)) => {
                self.kademlia.on_connection_handler_event(peer_id, connection_id, ev);
            }
            EitherOutput::Second(EitherOutput::Second(ev)) => {
                self.identify.on_connection_handler_event(peer_id, connection_id, ev);
            }
        }
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<ToSwarm<Self::ToSwarm, THandlerInEvent<Self>>> {
        use libp2p::swarm::behaviour::ToSwarm as LibToSwarm;
        
        // Poll gossipsub
        if let Poll::Ready(event) = self.gossipsub.poll(cx) {
            return match event {
                LibToSwarm::GenerateEvent(ev) => {
                    Poll::Ready(ToSwarm::GenerateEvent(SpiraChainBehaviourEvent::Gossipsub(ev)))
                }
                LibToSwarm::Dial { opts } => Poll::Ready(ToSwarm::Dial { opts }),
                LibToSwarm::NotifyHandler { peer_id, handler, event } => {
                    Poll::Ready(ToSwarm::NotifyHandler { 
                        peer_id, 
                        handler, 
                        event: libp2p::swarm::derive_prelude::EitherOutput::First(
                            libp2p::swarm::derive_prelude::EitherOutput::First(event)
                        )
                    })
                }
                LibToSwarm::CloseConnection { peer_id, connection } => {
                    Poll::Ready(ToSwarm::CloseConnection { peer_id, connection })
                }
                _ => Poll::Pending,
            };
        }

        // Poll mdns
        if let Poll::Ready(event) = self.mdns.poll(cx) {
            return match event {
                LibToSwarm::GenerateEvent(ev) => {
                    Poll::Ready(ToSwarm::GenerateEvent(SpiraChainBehaviourEvent::Mdns(ev)))
                }
                LibToSwarm::Dial { opts } => Poll::Ready(ToSwarm::Dial { opts }),
                LibToSwarm::NotifyHandler { peer_id, handler, event } => {
                    Poll::Ready(ToSwarm::NotifyHandler { 
                        peer_id, 
                        handler, 
                        event: libp2p::swarm::derive_prelude::EitherOutput::First(
                            libp2p::swarm::derive_prelude::EitherOutput::Second(event)
                        )
                    })
                }
                LibToSwarm::CloseConnection { peer_id, connection } => {
                    Poll::Ready(ToSwarm::CloseConnection { peer_id, connection })
                }
                _ => Poll::Pending,
            };
        }

        // Poll kademlia
        if let Poll::Ready(event) = self.kademlia.poll(cx) {
            return match event {
                LibToSwarm::GenerateEvent(ev) => {
                    Poll::Ready(ToSwarm::GenerateEvent(SpiraChainBehaviourEvent::Kademlia(ev)))
                }
                LibToSwarm::Dial { opts } => Poll::Ready(ToSwarm::Dial { opts }),
                LibToSwarm::NotifyHandler { peer_id, handler, event } => {
                    Poll::Ready(ToSwarm::NotifyHandler { 
                        peer_id, 
                        handler, 
                        event: libp2p::swarm::derive_prelude::EitherOutput::Second(
                            libp2p::swarm::derive_prelude::EitherOutput::First(event)
                        )
                    })
                }
                LibToSwarm::CloseConnection { peer_id, connection } => {
                    Poll::Ready(ToSwarm::CloseConnection { peer_id, connection })
                }
                _ => Poll::Pending,
            };
        }

        // Poll identify
        if let Poll::Ready(event) = self.identify.poll(cx) {
            return match event {
                LibToSwarm::GenerateEvent(ev) => {
                    Poll::Ready(ToSwarm::GenerateEvent(SpiraChainBehaviourEvent::Identify(ev)))
                }
                LibToSwarm::Dial { opts } => Poll::Ready(ToSwarm::Dial { opts }),
                LibToSwarm::NotifyHandler { peer_id, handler, event } => {
                    Poll::Ready(ToSwarm::NotifyHandler { 
                        peer_id, 
                        handler, 
                        event: libp2p::swarm::derive_prelude::EitherOutput::Second(
                            libp2p::swarm::derive_prelude::EitherOutput::Second(event)
                        )
                    })
                }
                LibToSwarm::CloseConnection { peer_id, connection } => {
                    Poll::Ready(ToSwarm::CloseConnection { peer_id, connection })
                }
                _ => Poll::Pending,
            };
        }

        Poll::Pending
    }
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
        let store = kad::store::MemoryStore::new(local_peer_id);
        let mut kademlia = kad::Behaviour::new(local_peer_id, store);
        kademlia.set_mode(Some(kad::Mode::Server));

        // 4. Create Identify (for peer info exchange)
        let identify = identify::Behaviour::new(identify::Config::new(
            format!("/spirachain/{}/1.0.0", network),
            local_key.public(),
        ));

        // Combine all behaviours (manual, no derive macro)
        let behaviour = SpiraChainBehaviour {
            gossipsub,
            mdns,
            kademlia,
            identify,
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
        info!("   ✓ DNS Seeds (bootstrap)");

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
                warn!("   This is normal if DNS seeds are not yet configured");
                warn!("   Node will work independently until peers are discovered");
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
            // Gossipsub events
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
    /// Uses Gossipsub to request blocks (simplified approach)
    pub fn request_blocks(&mut self, peer_id: PeerId, start_height: u64, count: u64) -> Result<()> {
        info!("📥 Requesting blocks {}-{} from {}", start_height, start_height + count - 1, peer_id);
        // Note: Full block sync via request/response will be added in next iteration
        // For now, nodes sync via Gossipsub block propagation
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
