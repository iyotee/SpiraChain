use spirachain_core::{Block, Transaction, Hash, BlockHeader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    NewBlock(Block),
    NewTransaction(Transaction),
    BlockRequest { start_height: u64, count: u64 },
    BlockResponse { blocks: Vec<Block> },
    HeaderRequest { start_height: u64, count: u64 },
    HeaderResponse { headers: Vec<BlockHeader> },
    SyncRequest { from_height: u64 },
    SyncResponse { blocks: Vec<Block>, has_more: bool },
    PeerInfo { peer_count: usize, chain_height: u64 },
    Ping,
    Pong,
}

impl NetworkMessage {
    pub fn new_block(block: Block) -> Self {
        NetworkMessage::NewBlock(block)
    }

    pub fn new_transaction(tx: Transaction) -> Self {
        NetworkMessage::NewTransaction(tx)
    }

    pub fn block_request(start_height: u64, count: u64) -> Self {
        NetworkMessage::BlockRequest { start_height, count }
    }

    pub fn sync_request(from_height: u64) -> Self {
        NetworkMessage::SyncRequest { from_height }
    }

    pub fn peer_info(peer_count: usize, chain_height: u64) -> Self {
        NetworkMessage::PeerInfo { peer_count, chain_height }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub syncing: bool,
    pub current_height: u64,
    pub target_height: u64,
    pub peers_syncing_from: usize,
}

impl SyncStatus {
    pub fn new() -> Self {
        Self {
            syncing: false,
            current_height: 0,
            target_height: 0,
            peers_syncing_from: 0,
        }
    }

    pub fn is_synced(&self) -> bool {
        !self.syncing && self.current_height >= self.target_height
    }

    pub fn progress_percent(&self) -> f64 {
        if self.target_height == 0 {
            return 100.0;
        }
        (self.current_height as f64 / self.target_height as f64) * 100.0
    }
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self::new()
    }
}
