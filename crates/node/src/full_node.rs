use spirachain_core::{Block, Transaction, Hash, Result, Amount, Address};
use spirachain_consensus::ProofOfSpiral;
use crate::{NodeConfig, Mempool, WorldState, BlockStorage};
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::time::{interval, Duration};
use tracing::{info, error};

pub struct FullNode {
    config: NodeConfig,
    mempool: Mempool,
    state: Arc<RwLock<WorldState>>,
    storage: BlockStorage,
    consensus: ProofOfSpiral,
    is_running: Arc<RwLock<bool>>,
}

impl FullNode {
    pub fn new(config: NodeConfig) -> Result<Self> {
        let storage = BlockStorage::new(&config.data_dir)?;
        let consensus = ProofOfSpiral::new(
            spirachain_core::MIN_SPIRAL_COMPLEXITY,
            spirachain_core::MAX_SPIRAL_JUMP
        );

        Ok(Self {
            config,
            mempool: Mempool::default(),
            state: Arc::new(RwLock::new(WorldState::default())),
            storage,
            consensus,
            is_running: Arc::new(RwLock::new(false)),
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 Starting SpiraChain Full Node");
        info!("   Data dir: {}", self.config.data_dir.display());
        info!("   RPC addr: {}", self.config.rpc_addr);

        *self.is_running.write() = true;

        let latest_block = self.storage.get_latest_block()?;
        if latest_block.is_some() {
            info!("   Latest block: {}", self.storage.get_chain_height()?);
        } else {
            info!("   No blocks in storage (syncing needed)");
        }

        self.run_sync_loop().await?;

        Ok(())
    }

    async fn run_sync_loop(&mut self) -> Result<()> {
        let mut block_check = interval(Duration::from_secs(30));
        let mut stats_interval = interval(Duration::from_secs(10));

        info!("⚡ Full node sync loop started");

        loop {
            tokio::select! {
                _ = block_check.tick() => {
                    if let Err(e) = self.check_for_new_blocks().await {
                        error!("Failed to check for new blocks: {}", e);
                    }
                }
                
                _ = stats_interval.tick() => {
                    self.print_stats();
                }
            }

            if !*self.is_running.read() {
                info!("Full node stopped");
                break;
            }
        }

        Ok(())
    }

    async fn check_for_new_blocks(&mut self) -> Result<()> {
        info!("Checking for new blocks...");
        Ok(())
    }

    pub async fn process_block(&mut self, block: Block) -> Result<()> {
        info!("Processing block at height {}", block.header.block_height);
        
        let latest_block = self.storage.get_latest_block()?;
        
        if let Some(prev_block) = latest_block {
            self.consensus.validate_block(&block, &prev_block)?;
        }

        self.storage.store_block(&block)?;

        {
            let mut state = self.state.write();
            for tx in &block.transactions {
                state.transfer(&tx.from, &tx.to, tx.amount)?;
                state.increment_nonce(&tx.from);
            }
            state.set_height(block.header.block_height);
        }

        info!("✅ Block {} validated and stored", block.header.block_height);

        Ok(())
    }

    pub async fn submit_transaction(&mut self, tx: Transaction) -> Result<()> {
        info!("Received transaction: {} → {} ({} QBT)", 
            tx.from.to_string()[..16].to_string(),
            tx.to.to_string()[..16].to_string(),
            tx.amount
        );

        tx.validate()?;
        self.mempool.add_transaction_sync(tx)?;
        
        Ok(())
    }

    pub fn get_balance(&self, address: &Address) -> Amount {
        self.state.read().get_balance(address)
    }

    pub async fn get_block_by_height(&self, height: u64) -> Result<Option<Block>> {
        self.storage.get_block_by_height(height)
    }

    pub async fn get_transaction_by_hash(&self, hash: &Hash) -> Result<Option<Transaction>> {
        self.storage.get_transaction(hash)
    }

    pub fn stop(&self) {
        *self.is_running.write() = false;
        info!("Stopping full node...");
    }

    fn print_stats(&self) {
        let height = self.storage.get_chain_height().unwrap_or(0);
        let mempool_size = self.mempool.size();
        let state = self.state.read();
        
        info!("📊 Full Node Stats: Height={} Mempool={} Accounts={}", 
            height, 
            mempool_size,
            state.account_count()
        );
    }
}
