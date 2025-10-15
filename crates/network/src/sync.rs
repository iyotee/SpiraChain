use spirachain_core::{Block, Result, SpiraChainError};
use std::collections::{HashMap, VecDeque};
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncState {
    Idle,
    Syncing,
    Synced,
}

pub struct SyncManager {
    current_height: u64,
    target_height: u64,
    state: SyncState,
    pending_blocks: HashMap<u64, Block>,
    sync_queue: VecDeque<u64>,
    last_sync_attempt: std::time::Instant,
}

impl SyncManager {
    pub fn new() -> Self {
        Self {
            current_height: 0,
            target_height: 0,
            state: SyncState::Idle,
            pending_blocks: HashMap::new(),
            sync_queue: VecDeque::new(),
            last_sync_attempt: std::time::Instant::now(),
        }
    }

    pub fn set_current_height(&mut self, height: u64) {
        self.current_height = height;
        info!("📊 Current blockchain height: {}", height);
    }

    pub fn update_target_height(&mut self, peer_height: u64) -> bool {
        if peer_height > self.target_height {
            let was_synced = self.state == SyncState::Synced;
            self.target_height = peer_height;
            
            if peer_height > self.current_height {
                self.state = SyncState::Syncing;
                info!("🔄 Sync target updated: {} → {} ({} blocks behind)", 
                    self.current_height, self.target_height, 
                    self.target_height - self.current_height);
                return true;
            } else if was_synced {
                self.state = SyncState::Synced;
            }
        }
        false
    }

    pub fn get_next_blocks_to_request(&mut self) -> Option<(u64, u64)> {
        if self.state != SyncState::Syncing {
            return None;
        }

        // Don't spam requests
        if self.last_sync_attempt.elapsed() < std::time::Duration::from_secs(5) {
            return None;
        }

        if self.current_height >= self.target_height {
            self.state = SyncState::Synced;
            info!("✅ Blockchain synchronized at height {}", self.current_height);
            return None;
        }

        let start = self.current_height + 1;
        let count = (self.target_height - self.current_height).min(100); // Request max 100 blocks at a time

        self.last_sync_attempt = std::time::Instant::now();
        Some((start, count))
    }

    pub fn add_pending_block(&mut self, block: Block) {
        let height = block.header.block_height;
        debug!("📦 Added pending block at height {}", height);
        self.pending_blocks.insert(height, block);
    }

    pub fn get_next_sequential_block(&mut self) -> Option<Block> {
        let next_height = self.current_height + 1;
        self.pending_blocks.remove(&next_height)
    }

    pub async fn fast_sync(&mut self, target_height: u64) -> Result<()> {
        self.target_height = target_height;
        self.state = SyncState::Syncing;
        info!("🚀 Starting fast sync to height {}", target_height);

        Ok(())
    }

    pub async fn sync_block(&mut self, block: Block) -> Result<()> {
        let block_height = block.header.block_height;
        
        if block_height == self.current_height + 1 {
            info!("✅ Synced block at height {}", block_height);
            self.current_height = block_height;
            
            // Check if we've caught up
            if self.current_height >= self.target_height {
                self.state = SyncState::Synced;
                info!("🎉 Blockchain fully synchronized at height {}", self.current_height);
            }
        } else if block_height > self.current_height + 1 {
            // Out of order - add to pending
            self.add_pending_block(block);
        } else {
            // Already have this block
            debug!("Skipping already synced block at height {}", block_height);
        }

        Ok(())
    }

    pub fn is_synced(&self) -> bool {
        self.state == SyncState::Synced || self.current_height >= self.target_height
    }

    pub fn is_syncing(&self) -> bool {
        self.state == SyncState::Syncing
    }

    pub fn sync_progress(&self) -> f64 {
        if self.target_height == 0 {
            return 1.0;
        }
        (self.current_height as f64) / (self.target_height as f64)
    }

    pub fn get_current_height(&self) -> u64 {
        self.current_height
    }

    pub fn get_target_height(&self) -> u64 {
        self.target_height
    }

    pub fn get_state(&self) -> SyncState {
        self.state
    }
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new()
    }
}
