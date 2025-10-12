use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, Swarm, Transport,
    futures::StreamExt,
};
use std::collections::HashSet;
use std::error::Error;
use std::time::Duration;
use tokio::select;
use tracing::{info, warn};
use crate::protocol::NetworkMessage;

#[derive(NetworkBehaviour)]
pub struct SpiraChainBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub struct P2PNetwork {
    swarm: Swarm<SpiraChainBehaviour>,
    known_peers: HashSet<PeerId>,
}

impl P2PNetwork {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        info!("Local peer id: {}", local_peer_id);

        let transport = tcp::tokio::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::Config::default())
            .timeout(Duration::from_secs(20))
            .boxed();

        let message_id_fn = |message: &gossipsub::Message| {
            let mut hasher = blake3::Hasher::new();
            hasher.update(&message.data);
            gossipsub::MessageId::from(hasher.finalize().as_bytes())
        };

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            .build()
            .expect("Valid config");

        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )?;

        let block_topic = gossipsub::IdentTopic::new("spirachain/blocks");
        let tx_topic = gossipsub::IdentTopic::new("spirachain/transactions");
        
        gossipsub.subscribe(&block_topic)?;
        gossipsub.subscribe(&tx_topic)?;

        let mdns = mdns::tokio::Behaviour::new(
            mdns::Config::default(),
            local_peer_id,
        )?;

        let behaviour = SpiraChainBehaviour { gossipsub, mdns };

        let swarm = Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            libp2p::swarm::Config::with_tokio_executor(),
        );

        Ok(Self {
            swarm,
            known_peers: HashSet::new(),
        })
    }

    pub async fn listen(&mut self, port: u16) -> Result<(), Box<dyn Error>> {
        let addr = format!("/ip4/0.0.0.0/tcp/{}", port).parse()?;
        self.swarm.listen_on(addr)?;
        info!("Listening on port {}", port);
        Ok(())
    }

    pub fn broadcast_block(&mut self, block_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let topic = gossipsub::IdentTopic::new("spirachain/blocks");
        self.swarm.behaviour_mut().gossipsub.publish(topic, block_data)?;
        Ok(())
    }

    pub fn broadcast_transaction(&mut self, tx_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let topic = gossipsub::IdentTopic::new("spirachain/transactions");
        self.swarm.behaviour_mut().gossipsub.publish(topic, tx_data)?;
        Ok(())
    }

    pub async fn run_event_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            select! {
                event = self.swarm.select_next_some() => {
                    match event {
                        SwarmEvent::Behaviour(SpiraChainBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                            for (peer_id, _multiaddr) in list {
                                info!("Discovered peer: {}", peer_id);
                                self.known_peers.insert(peer_id);
                                
                                let _ = self.swarm
                                    .behaviour_mut()
                                    .gossipsub
                                    .add_explicit_peer(&peer_id);
                            }
                        }
                        SwarmEvent::Behaviour(SpiraChainBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                            for (peer_id, _multiaddr) in list {
                                info!("Peer expired: {}", peer_id);
                                self.known_peers.remove(&peer_id);
                                
                                self.swarm
                                    .behaviour_mut()
                                    .gossipsub
                                    .remove_explicit_peer(&peer_id);
                            }
                        }
                        SwarmEvent::Behaviour(SpiraChainBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source: peer_id,
                            message_id: id,
                            message,
                        })) => {
                            info!(
                                "Got message: '{}' with id: {} from peer: {}",
                                String::from_utf8_lossy(&message.data),
                                id,
                                peer_id
                            );
                            
                            if let Ok(msg) = serde_json::from_slice::<NetworkMessage>(&message.data) {
                                self.handle_network_message(msg).await;
                            }
                        }
                        SwarmEvent::NewListenAddr { address, .. } => {
                            info!("Listening on {}", address);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    async fn handle_network_message(&mut self, msg: NetworkMessage) {
        match msg {
            NetworkMessage::NewBlock(_block_data) => {
                info!("Received new block");
            }
            NetworkMessage::NewTransaction(_tx_data) => {
                info!("Received new transaction");
            }
            NetworkMessage::SpiralValidationRequest(_data) => {
                info!("Received spiral validation request");
            }
            NetworkMessage::SemanticQuery(query) => {
                info!("Received semantic query: {}", query);
            }
            NetworkMessage::PeerInfo(peer_info) => {
                info!("Received peer info from {}", peer_info.peer_id);
            }
            NetworkMessage::SyncRequest(sync_req) => {
                info!("Sync request: blocks {}-{}", sync_req.start_height, sync_req.end_height);
            }
            NetworkMessage::SyncResponse(sync_resp) => {
                info!("Sync response: {} blocks", sync_resp.blocks.len());
            }
        }
    }

    pub fn peer_count(&self) -> usize {
        self.known_peers.len()
    }

    pub fn get_peers(&self) -> Vec<PeerId> {
        self.known_peers.iter().cloned().collect()
    }
}
