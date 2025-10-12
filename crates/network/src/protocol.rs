use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u32 = 1;

pub const PROTOCOLS: [&str; 5] = [
    "/spirachain/block/1.0.0",
    "/spirachain/tx/1.0.0",
    "/spirachain/spiral/1.0.0",
    "/spirachain/semantic/1.0.0",
    "/spirachain/sync/1.0.0",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    NewBlock(Vec<u8>),
    NewTransaction(Vec<u8>),
    SpiralValidationRequest(Vec<u8>),
    SemanticQuery(String),
    PeerInfo(PeerInfoMessage),
    SyncRequest(SyncRequestMessage),
    SyncResponse(SyncResponseMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfoMessage {
    pub peer_id: String,
    pub chain_height: u64,
    pub best_block_hash: String,
    pub validator_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequestMessage {
    pub start_height: u64,
    pub end_height: u64,
    pub max_blocks: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponseMessage {
    pub blocks: Vec<Vec<u8>>,
    pub has_more: bool,
}

pub struct ProtocolHandler;

impl ProtocolHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn supports_protocol(&self, protocol: &str) -> bool {
        PROTOCOLS.contains(&protocol)
    }
}

impl Default for ProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}
