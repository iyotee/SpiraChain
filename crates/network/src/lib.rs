pub mod encryption;
pub mod libp2p_v53;
pub mod p2p;
pub mod protocol;
pub mod sync; // LibP2P v0.53 implementation

pub use encryption::*;
pub use libp2p_v53::LibP2PNetwork;
pub use p2p::*;
pub use protocol::*;
pub use sync::*;

use spirachain_core::{Block, Result, Transaction};

pub struct NetworkNode {
    #[allow(dead_code)]
    peer_id: String,
    listening_addr: String,
}

impl NetworkNode {
    pub fn new(listening_addr: String) -> Self {
        Self {
            peer_id: "peer-placeholder".to_string(),
            listening_addr,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Network node starting on {}", self.listening_addr);
        Ok(())
    }

    pub async fn broadcast_block(&self, block: &Block) -> Result<()> {
        tracing::info!("Broadcasting block {}", block.hash());
        Ok(())
    }

    pub async fn broadcast_transaction(&self, tx: &Transaction) -> Result<()> {
        tracing::info!("Broadcasting transaction {}", tx.tx_hash);
        Ok(())
    }

    pub async fn get_peers(&self) -> Vec<String> {
        vec![]
    }
}
