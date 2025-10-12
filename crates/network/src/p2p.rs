use spirachain_core::{Block, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    BlockAnnouncement(BlockAnnouncement),
    TransactionGossip(TransactionGossip),
    SpiralValidationRequest(SpiralValidationRequest),
    SemanticQuery(SemanticQuery),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockAnnouncement {
    pub block_hash: Vec<u8>,
    pub block_height: u64,
    pub validator: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionGossip {
    pub tx_hash: Vec<u8>,
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub amount: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiralValidationRequest {
    pub block_hash: Vec<u8>,
    pub validation_level: ValidationLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ValidationLevel {
    Quick,
    Geometric,
    Semantic,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticQuery {
    pub query_text: String,
    pub max_results: usize,
}

pub struct PeerManager {
    peers: Vec<PeerInfo>,
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub peer_id: String,
    pub address: String,
    pub reputation: f64,
    pub last_seen: u64,
}

impl PeerManager {
    pub fn new() -> Self {
        Self {
            peers: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, peer: PeerInfo) {
        self.peers.push(peer);
    }

    pub fn get_peers(&self) -> &[PeerInfo] {
        &self.peers
    }

    pub fn get_best_peers(&self, count: usize) -> Vec<&PeerInfo> {
        let mut peers: Vec<&PeerInfo> = self.peers.iter().collect();
        peers.sort_by(|a, b| b.reputation.partial_cmp(&a.reputation).unwrap());
        peers.into_iter().take(count).collect()
    }
}

impl Default for PeerManager {
    fn default() -> Self {
        Self::new()
    }
}

