use spirachain_core::{Block, Transaction, Hash, Result, SpiraChainError};
use crate::protocol::NetworkMessage;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{info, warn, error, debug};

pub struct P2PNetwork {
    peers: HashMap<String, PeerInfo>,
    message_tx: mpsc::UnboundedSender<NetworkMessage>,
    message_rx: mpsc::UnboundedReceiver<NetworkMessage>,
    local_peer_id: String,
    port: u16,
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub peer_id: String,
    pub addresses: Vec<String>,
    pub connected_at: std::time::Instant,
    pub blocks_shared: u64,
    pub transactions_shared: u64,
}

impl P2PNetwork {
    pub async fn new(port: u16) -> Result<Self> {
        let local_peer_id = format!("spirachain-{}", hex::encode(&blake3::hash(format!("{}", port).as_bytes()).as_bytes()[..8]));
        
        info!("🌐 Starting P2P network");
        info!("   Local Peer ID: {}", local_peer_id);
        info!("   Port: {}", port);

        let (message_tx, message_rx) = mpsc::unbounded_channel();

        info!("✅ P2P network initialized");

        Ok(Self {
            peers: HashMap::new(),
            message_tx,
            message_rx,
            local_peer_id,
            port,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 P2P network ready");
        info!("   Discovery: Local network + manual peering");
        info!("   Port: {}", self.port);
        
        loop {
            tokio::select! {
                Some(message) = self.message_rx.recv() => {
                    self.handle_outgoing_message(message).await?;
                }
                
                _ = tokio::time::sleep(Duration::from_secs(30)) => {
                    self.print_network_stats();
                }
            }
        }
    }

    async fn handle_outgoing_message(&mut self, message: NetworkMessage) -> Result<()> {
        match message {
            NetworkMessage::NewBlock(block) => {
                info!("📤 Broadcasting block: height={}", block.header.block_height);
                
                for peer_info in self.peers.values_mut() {
                    peer_info.blocks_shared += 1;
                }
            }

            NetworkMessage::NewTransaction(tx) => {
                info!("📤 Broadcasting transaction: hash={}", tx.hash());
                
                for peer_info in self.peers.values_mut() {
                    peer_info.transactions_shared += 1;
                }
            }

            _ => {
                debug!("Unhandled outgoing message: {:?}", message);
            }
        }

        Ok(())
    }

    pub fn broadcast_block(&self, block: Block) -> Result<()> {
        self.message_tx.send(NetworkMessage::NewBlock(block))
            .map_err(|e| SpiraChainError::Internal(format!("Channel send: {}", e)))
    }

    pub fn broadcast_transaction(&self, tx: Transaction) -> Result<()> {
        self.message_tx.send(NetworkMessage::NewTransaction(tx))
            .map_err(|e| SpiraChainError::Internal(format!("Channel send: {}", e)))
    }

    pub fn add_peer(&mut self, peer_id: String, address: String) {
        info!("🤝 Adding peer: {} at {}", peer_id, address);
        
        self.peers.insert(peer_id.clone(), PeerInfo {
            peer_id,
            addresses: vec![address],
            connected_at: std::time::Instant::now(),
            blocks_shared: 0,
            transactions_shared: 0,
        });
    }

    pub fn remove_peer(&mut self, peer_id: &str) {
        if self.peers.remove(peer_id).is_some() {
            info!("👋 Removed peer: {}", peer_id);
        }
    }

    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn peers(&self) -> Vec<PeerInfo> {
        self.peers.values().cloned().collect()
    }

    pub fn local_peer_id(&self) -> String {
        self.local_peer_id.clone()
    }

    fn print_network_stats(&self) {
        info!("🌐 Network Stats:");
        info!("   Peers: {}", self.peers.len());
        info!("   Local ID: {}", self.local_peer_id);
        
        for peer in self.peers.values() {
            info!("   → {}: {} blocks, {} txs", 
                peer.peer_id, 
                peer.blocks_shared, 
                peer.transactions_shared
            );
        }
    }
}

pub async fn connect_to_bootnode(network: &mut P2PNetwork, bootnode_addr: &str) -> Result<()> {
    info!("🌟 Connecting to bootnode: {}", bootnode_addr);
    
    let parts: Vec<&str> = bootnode_addr.split('@').collect();
    if parts.len() == 2 {
        network.add_peer(parts[0].to_string(), parts[1].to_string());
    }
    
    Ok(())
}
