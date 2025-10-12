use spirachain_core::{Block, Hash, Result};

pub struct SyncManager {
    current_height: u64,
    target_height: u64,
}

impl SyncManager {
    pub fn new() -> Self {
        Self {
            current_height: 0,
            target_height: 0,
        }
    }

    pub async fn fast_sync(&mut self, target_height: u64) -> Result<()> {
        self.target_height = target_height;
        tracing::info!("Starting fast sync to height {}", target_height);
        
        Ok(())
    }

    pub async fn sync_block(&mut self, block: Block) -> Result<()> {
        tracing::info!("Syncing block at height {}", block.header.block_height);
        self.current_height = block.header.block_height;
        Ok(())
    }

    pub fn is_synced(&self) -> bool {
        self.current_height >= self.target_height
    }

    pub fn sync_progress(&self) -> f64 {
        if self.target_height == 0 {
            return 1.0;
        }
        (self.current_height as f64) / (self.target_height as f64)
    }
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new()
    }
}

