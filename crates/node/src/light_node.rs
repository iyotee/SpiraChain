use spirachain_core::{BlockHeader, Hash, Result};
use crate::NodeConfig;
use std::collections::HashMap;
use tracing::info;

pub struct LightNode {
    config: NodeConfig,
    headers: HashMap<u64, BlockHeader>,
    latest_height: u64,
}

impl LightNode {
    pub fn new(config: NodeConfig) -> Result<Self> {
        Ok(Self {
            config,
            headers: HashMap::new(),
            latest_height: 0,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 Starting SpiraChain Light Node");
        info!("   Data dir: {}", self.config.data_dir.display());
        info!("   Network: {}", self.config.network_addr);

        info!("⚡ Light node started (header-only mode)");

        Ok(())
    }

    pub async fn process_header(&mut self, header: BlockHeader) -> Result<()> {
        info!("Received header for block {}", header.block_height);
        
        if header.block_height > self.latest_height {
            self.headers.insert(header.block_height, header.clone());
            self.latest_height = header.block_height;
            
            info!("✅ Header {} stored (latest: {})", 
                header.block_height,
                self.latest_height
            );
        }

        Ok(())
    }

    pub fn get_header(&self, height: u64) -> Option<&BlockHeader> {
        self.headers.get(&height)
    }

    pub fn current_height(&self) -> u64 {
        self.latest_height
    }

    pub fn verify_spv_proof(&self, tx_hash: &Hash, proof: &[Hash], block_height: u64) -> bool {
        if let Some(header) = self.headers.get(&block_height) {
            let computed_root = Self::compute_merkle_root_from_proof(tx_hash, proof);
            computed_root == header.merkle_root
        } else {
            false
        }
    }

    fn compute_merkle_root_from_proof(tx_hash: &Hash, proof: &[Hash]) -> Hash {
        let mut current = *tx_hash;
        
        for sibling in proof {
            let mut combined = Vec::with_capacity(64);
            combined.extend_from_slice(current.as_bytes());
            combined.extend_from_slice(sibling.as_bytes());
            current = Hash::from(blake3::hash(&combined));
        }
        
        current
    }
}
