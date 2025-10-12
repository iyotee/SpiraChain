use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketMessage {
    NewBlock { height: u64, hash: String },
    NewTransaction { hash: String },
    NetworkStatus { peer_count: usize },
}

pub struct WebSocketManager {
    connections: Vec<String>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }

    pub async fn broadcast(&self, message: WebSocketMessage) {
        tracing::debug!("Broadcasting WebSocket message: {:?}", message);
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

