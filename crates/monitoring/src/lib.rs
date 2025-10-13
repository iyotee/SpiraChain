use parking_lot::RwLock;
use spirachain_core::Result;
use std::sync::Arc;

pub struct SpiraChainMetrics {
    pub blocks_produced: Arc<RwLock<u64>>,
    pub blocks_validated: Arc<RwLock<u64>>,
    pub transactions_processed: Arc<RwLock<u64>>,
    pub peer_count: Arc<RwLock<usize>>,
    pub chain_height: Arc<RwLock<u64>>,
}

impl SpiraChainMetrics {
    pub fn new() -> Self {
        Self {
            blocks_produced: Arc::new(RwLock::new(0)),
            blocks_validated: Arc::new(RwLock::new(0)),
            transactions_processed: Arc::new(RwLock::new(0)),
            peer_count: Arc::new(RwLock::new(0)),
            chain_height: Arc::new(RwLock::new(0)),
        }
    }

    pub fn export_prometheus(&self) -> String {
        format!(
            "# HELP spirachain_blocks_produced Total blocks produced\n\
             # TYPE spirachain_blocks_produced counter\n\
             spirachain_blocks_produced {}\n\
             # HELP spirachain_blocks_validated Total blocks validated\n\
             # TYPE spirachain_blocks_validated counter\n\
             spirachain_blocks_validated {}\n\
             # HELP spirachain_transactions Total transactions\n\
             # TYPE spirachain_transactions counter\n\
             spirachain_transactions {}\n\
             # HELP spirachain_peers Connected peers\n\
             # TYPE spirachain_peers gauge\n\
             spirachain_peers {}\n\
             # HELP spirachain_height Chain height\n\
             # TYPE spirachain_height gauge\n\
             spirachain_height {}\n",
            *self.blocks_produced.read(),
            *self.blocks_validated.read(),
            *self.transactions_processed.read(),
            *self.peer_count.read(),
            *self.chain_height.read(),
        )
    }

    pub fn record_block_produced(&self) {
        *self.blocks_produced.write() += 1;
    }

    pub fn record_block_validated(&self) {
        *self.blocks_validated.write() += 1;
    }

    pub fn record_transactions(&self, count: u64) {
        *self.transactions_processed.write() += count;
    }

    pub fn update_peer_count(&self, count: usize) {
        *self.peer_count.write() = count;
    }

    pub fn update_chain_height(&self, height: u64) {
        *self.chain_height.write() = height;
    }
}

impl Default for SpiraChainMetrics {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn start_metrics_server(metrics: Arc<SpiraChainMetrics>, port: u16) -> Result<()> {
    use warp::Filter;

    tracing::info!("📊 Starting metrics server on port {}", port);

    let metrics_clone = metrics.clone();
    let metrics_route = warp::path("metrics").map(move || {
        let output = metrics_clone.export_prometheus();
        warp::reply::with_header(output, "Content-Type", "text/plain")
    });

    warp::serve(metrics_route).run(([0, 0, 0, 0], port)).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = SpiraChainMetrics::new();
        assert_eq!(*metrics.blocks_produced.read(), 0);
    }

    #[test]
    fn test_record_block() {
        let metrics = SpiraChainMetrics::new();
        metrics.record_block_produced();
        metrics.record_block_produced();
        assert_eq!(*metrics.blocks_produced.read(), 2);
    }

    #[test]
    fn test_prometheus_export() {
        let metrics = SpiraChainMetrics::new();
        metrics.record_block_produced();
        metrics.update_chain_height(12345);

        let export = metrics.export_prometheus();
        assert!(export.contains("spirachain_blocks_produced 1"));
        assert!(export.contains("spirachain_height 12345"));
    }
}
