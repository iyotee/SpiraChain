use spirachain_core::{BlockHeader, Hash, Result};
use crate::NodeConfig;
use std::collections::HashMap;

pub struct LightNode {
    config: NodeConfig,
    headers: HashMap<Hash, BlockHeader>,
    current_height: u64,
}

impl LightNode {
    pub fn new(config: NodeConfig) -> Self {
        Self {
            config,
            headers: HashMap::new(),
            current_height: 0,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting light node...");
        tracing::info!("Network address: {}", self.config.network_addr);

        Ok(())
    }

    pub async fn sync_headers(&mut self) -> Result<()> {
        tracing::info!("Syncing headers...");
        Ok(())
    }

    pub fn get_header(&self, hash: &Hash) -> Option<&BlockHeader> {
        self.headers.get(hash)
    }

    pub fn current_height(&self) -> u64 {
        self.current_height
    }
}

