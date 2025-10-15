pub mod bootstrap;
pub mod encryption;
pub mod libp2p_v53;
pub mod libp2p_sync;
pub mod p2p;
pub mod protocol;
pub mod sync;

pub use bootstrap::*;
pub use encryption::*;
pub use libp2p_v53::LibP2PNetwork;
pub use libp2p_sync::{LibP2PNetworkWithSync, NetworkEvent};
pub use p2p::*;
pub use protocol::*;
pub use sync::*;

use spirachain_core::{Block, Result, Transaction};

pub struct NetworkNode {
    peer_id: String,
    listening_addr: String,
}

impl NetworkNode {
    pub fn new(listening_addr: String) -> Self {
        // Generate a unique peer ID based on the listening address
        let peer_id = format!("peer-{}", blake3::hash(listening_addr.as_bytes()).to_hex());
        Self {
            peer_id,
            listening_addr,
        }
    }

    /// Get the peer ID of this node
    pub fn peer_id(&self) -> &str {
        &self.peer_id
    }

    /// Get the listening address
    pub fn listening_addr(&self) -> &str {
        &self.listening_addr
    }

    pub async fn start(&mut self) -> Result<()> {
        tracing::info!(
            "🌐 Network node starting - Peer ID: {} on {}",
            self.peer_id,
            self.listening_addr
        );
        Ok(())
    }

    pub async fn broadcast_block(&self, block: &Block) -> Result<()> {
        tracing::info!("📡 [{}] Broadcasting block {}", self.peer_id, block.hash());
        Ok(())
    }

    pub async fn broadcast_transaction(&self, tx: &Transaction) -> Result<()> {
        tracing::info!(
            "📡 [{}] Broadcasting transaction {}",
            self.peer_id,
            tx.tx_hash
        );
        Ok(())
    }

    pub async fn get_peers(&self) -> Vec<String> {
        // Return empty for now - would be populated by LibP2P discovery
        vec![]
    }
}
