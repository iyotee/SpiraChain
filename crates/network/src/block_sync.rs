// Block Synchronization Protocol for SpiraChain
// Implements Bitcoin-style block download and validation

use libp2p::{
    request_response::{self, OutboundRequestId, ProtocolSupport, ResponseChannel},
    PeerId,
};
use serde::{Deserialize, Serialize};
use spirachain_core::{Block, Hash, Result, SpiraChainError};
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn};

// Protocol version
pub const PROTOCOL_VERSION: &str = "/spirachain/sync/1.0.0";

// Block sync messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockSyncRequest {
    // Get blockchain height
    GetHeight,
    
    // Get block headers from start_height to end_height (max 500)
    GetHeaders {
        start_height: u64,
        end_height: u64,
    },
    
    // Get full blocks by hash (max 10 blocks per request)
    GetBlocks {
        hashes: Vec<Hash>,
    },
    
    // Get blocks from height (max 10 blocks)
    GetBlocksByHeight {
        start_height: u64,
        count: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockSyncResponse {
    // Blockchain height
    Height {
        height: u64,
        best_hash: Hash,
    },
    
    // Block headers
    Headers {
        headers: Vec<BlockHeader>,
    },
    
    // Full blocks
    Blocks {
        blocks: Vec<Block>,
    },
    
    // Error
    Error {
        message: String,
    },
}

// Lightweight block header for sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub height: u64,
    pub hash: Hash,
    pub previous_hash: Hash,
    pub timestamp: u64,
}

// Codec for request/response
#[derive(Debug, Clone)]
pub struct BlockSyncCodec;

impl request_response::Codec for BlockSyncCodec {
    type Protocol = String;
    type Request = BlockSyncRequest;
    type Response = BlockSyncResponse;

    async fn read_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> std::io::Result<Self::Request>
    where
        T: futures::AsyncRead + Unpin + Send,
    {
        use futures::AsyncReadExt;
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        bincode::deserialize(&buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    async fn read_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> std::io::Result<Self::Response>
    where
        T: futures::AsyncRead + Unpin + Send,
    {
        use futures::AsyncReadExt;
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        bincode::deserialize(&buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    async fn write_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> std::io::Result<()>
    where
        T: futures::AsyncWrite + Unpin + Send,
    {
        use futures::AsyncWriteExt;
        let data = bincode::serialize(&req)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        io.write_all(&data).await?;
        io.close().await
    }

    async fn write_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> std::io::Result<()>
    where
        T: futures::AsyncWrite + Unpin + Send,
    {
        use futures::AsyncWriteExt;
        let data = bincode::serialize(&res)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        io.write_all(&data).await?;
        io.close().await
    }
}

// Block Sync Manager
pub struct BlockSyncManager {
    // Peers and their heights
    peer_heights: HashMap<PeerId, u64>,
    
    // Pending block requests
    pending_requests: HashMap<OutboundRequestId, PendingRequest>,
    
    // Downloaded blocks waiting for validation
    downloaded_blocks: HashMap<u64, Block>,
    
    // Current sync state
    sync_state: SyncState,
    
    // Local blockchain height
    local_height: u64,
    
    // Target height to sync to
    target_height: u64,
    
    // Best peer (highest height)
    best_peer: Option<PeerId>,
}

#[derive(Debug, Clone)]
struct PendingRequest {
    peer: PeerId,
    request_type: RequestType,
    timestamp: std::time::Instant,
}

#[derive(Debug, Clone)]
enum RequestType {
    Height,
    Headers { start: u64, end: u64 },
    Blocks { hashes: Vec<Hash> },
    BlocksByHeight { start: u64, count: u64 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncState {
    Idle,
    DiscoveringPeers,
    DownloadingHeaders { current: u64, target: u64 },
    DownloadingBlocks { current: u64, target: u64 },
    Validating,
    Synced,
}

impl BlockSyncManager {
    pub fn new(local_height: u64) -> Self {
        Self {
            peer_heights: HashMap::new(),
            pending_requests: HashMap::new(),
            downloaded_blocks: HashMap::new(),
            sync_state: SyncState::Idle,
            local_height,
            target_height: local_height,
            best_peer: None,
        }
    }

    /// Update local blockchain height
    pub fn set_local_height(&mut self, height: u64) {
        self.local_height = height;
        if height >= self.target_height {
            self.sync_state = SyncState::Synced;
        }
    }

    /// Register a peer's blockchain height
    pub fn register_peer_height(&mut self, peer: PeerId, height: u64) {
        info!("📊 Peer {} reported height: {}", peer, height);
        self.peer_heights.insert(peer, height);
        
        // Update best peer if this one is better
        if height > self.target_height {
            self.target_height = height;
            self.best_peer = Some(peer);
            info!(
                "🎯 New sync target: height {} from peer {}",
                height, peer
            );
        }
    }

    /// Get the current sync state
    pub fn get_state(&self) -> &SyncState {
        &self.sync_state
    }

    /// Check if we need to sync
    pub fn needs_sync(&self) -> bool {
        self.target_height > self.local_height
    }

    /// Get the next batch of blocks to download
    pub fn get_next_download_batch(&mut self) -> Option<(PeerId, BlockSyncRequest)> {
        if !self.needs_sync() {
            return None;
        }

        if let Some(best_peer) = self.best_peer {
            // Download blocks in batches of 10
            let start_height = self.local_height + 1;
            let count = std::cmp::min(10, self.target_height - self.local_height);

            self.sync_state = SyncState::DownloadingBlocks {
                current: start_height,
                target: self.target_height,
            };

            let request = BlockSyncRequest::GetBlocksByHeight {
                start_height,
                count,
            };

            Some((best_peer, request))
        } else {
            None
        }
    }

    /// Add a downloaded block
    pub fn add_downloaded_block(&mut self, block: Block) {
        let height = block.header.block_height;
        debug!("📦 Downloaded block at height {}", height);
        self.downloaded_blocks.insert(height, block);
    }

    /// Get next sequential block to validate
    pub fn get_next_block_to_validate(&mut self) -> Option<Block> {
        let next_height = self.local_height + 1;
        self.downloaded_blocks.remove(&next_height)
    }

    /// Mark a block as validated and increase local height
    pub fn mark_block_validated(&mut self, height: u64) {
        if height == self.local_height + 1 {
            self.local_height = height;
            debug!("✅ Block {} validated, local height now {}", height, height);
        }
    }

    /// Record a pending request
    pub fn record_request(
        &mut self,
        request_id: OutboundRequestId,
        peer: PeerId,
        request_type: RequestType,
    ) {
        self.pending_requests.insert(
            request_id,
            PendingRequest {
                peer,
                request_type,
                timestamp: std::time::Instant::now(),
            },
        );
    }

    /// Handle a response
    pub fn handle_response(&mut self, request_id: OutboundRequestId) -> Option<PendingRequest> {
        self.pending_requests.remove(&request_id)
    }

    /// Clean up timed-out requests (> 30 seconds)
    pub fn cleanup_timeouts(&mut self) {
        let now = std::time::Instant::now();
        self.pending_requests.retain(|_id, pending| {
            let elapsed = now.duration_since(pending.timestamp);
            if elapsed.as_secs() > 30 {
                warn!("⏱️ Request to {} timed out", pending.peer);
                false
            } else {
                true
            }
        });
    }

    /// Reset sync state (e.g., after fork detected)
    pub fn reset(&mut self) {
        warn!("🔄 Resetting sync state");
        self.downloaded_blocks.clear();
        self.pending_requests.clear();
        self.sync_state = SyncState::Idle;
        self.target_height = self.local_height;
        self.best_peer = None;
    }

    /// Handle a fork (reorg needed)
    pub fn handle_fork(&mut self, fork_height: u64) {
        warn!("🍴 Fork detected at height {}", fork_height);
        
        // Rollback to fork point
        if fork_height < self.local_height {
            self.local_height = fork_height;
            
            // Clear downloaded blocks after fork point
            self.downloaded_blocks.retain(|&height, _| height <= fork_height);
            
            // Re-sync from fork point
            self.sync_state = SyncState::Idle;
        }
    }

    /// Get sync progress percentage
    pub fn get_progress(&self) -> f64 {
        if self.target_height == 0 {
            return 100.0;
        }
        
        (self.local_height as f64 / self.target_height as f64) * 100.0
    }

    /// Get sync statistics
    pub fn get_stats(&self) -> SyncStats {
        SyncStats {
            local_height: self.local_height,
            target_height: self.target_height,
            pending_requests: self.pending_requests.len(),
            downloaded_blocks: self.downloaded_blocks.len(),
            connected_peers: self.peer_heights.len(),
            progress: self.get_progress(),
            state: self.sync_state.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyncStats {
    pub local_height: u64,
    pub target_height: u64,
    pub pending_requests: usize,
    pub downloaded_blocks: usize,
    pub connected_peers: usize,
    pub progress: f64,
    pub state: SyncState,
}

impl std::fmt::Display for SyncStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sync: {:.1}% ({}/{}) | Peers: {} | Downloaded: {} | Pending: {} | State: {:?}",
            self.progress,
            self.local_height,
            self.target_height,
            self.connected_peers,
            self.downloaded_blocks,
            self.pending_requests,
            self.state
        )
    }
}

