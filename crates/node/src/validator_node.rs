use spirachain_core::{Block, Transaction, Hash, Result, Amount, Address};
use spirachain_crypto::KeyPair;
use spirachain_consensus::{ProofOfSpiral, Validator};
use crate::{NodeConfig, Mempool, WorldState, BlockStorage};
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::time::{interval, Duration};
use tracing::{info, error, warn};

pub struct ValidatorNode {
    config: NodeConfig,
    keypair: KeyPair,
    validator: Validator,
    mempool: Mempool,
    state: Arc<RwLock<WorldState>>,
    storage: BlockStorage,
    consensus: ProofOfSpiral,
    is_running: Arc<RwLock<bool>>,
    blocks_produced: u64,
}

impl ValidatorNode {
    pub fn new(config: NodeConfig, keypair: KeyPair) -> Result<Self> {
        let storage = BlockStorage::new(&config.data_dir)?;
        let address = keypair.to_address();
        
        let validator = Validator {
            address,
            pubkey: keypair.public_key().as_bytes().to_vec(),
            stake: Amount::new(10_000 * 10u128.pow(18)),
            locked_until: 0,
            rewards_earned: Amount::new(0),
            slashing_events: Vec::new(),
            blocks_proposed: 0,
            expected_blocks: 0,
            reputation_score: 1.0,
            last_block_height: 0,
        };

        let consensus = ProofOfSpiral::new(
            spirachain_core::MIN_SPIRAL_COMPLEXITY,
            spirachain_core::MAX_SPIRAL_JUMP
        );

        Ok(Self {
            config,
            keypair,
            validator,
            mempool: Mempool::default(),
            state: Arc::new(RwLock::new(WorldState::default())),
            storage,
            consensus,
            is_running: Arc::new(RwLock::new(false)),
            blocks_produced: 0,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 Starting SpiraChain Validator Node");
        info!("   Address: {}", self.validator.address);
        info!("   Stake: {} QBT", self.validator.stake.value() as f64 / 1e18);
        info!("   Data dir: {}", self.config.data_dir.display());

        *self.is_running.write() = true;

        let latest_block = self.storage.get_latest_block()?;
        if let Some(block) = latest_block {
            info!("   Latest block: {}", block.header.block_height);
            self.state.write().set_height(block.header.block_height);
        } else {
            info!("   No blocks yet - will create genesis");
        }

        self.run_validator_loop().await?;

        Ok(())
    }

    async fn run_validator_loop(&mut self) -> Result<()> {
        let mut block_timer = interval(Duration::from_secs(60));
        let mut stats_timer = interval(Duration::from_secs(30));
        let mut mempool_check = interval(Duration::from_secs(5));

        info!("⚡ Validator loop started (producing blocks every 60s)");

        loop {
            tokio::select! {
                _ = block_timer.tick() => {
                    if let Err(e) = self.produce_block().await {
                        error!("Failed to produce block: {}", e);
                    }
                }

                _ = stats_timer.tick() => {
                    self.print_stats();
                }

                _ = mempool_check.tick() => {
                    self.check_mempool();
                }
            }

            if !*self.is_running.read() {
                info!("Validator stopped");
                break;
            }
        }

        Ok(())
    }

    async fn produce_block(&mut self) -> Result<()> {
        info!("🏗️  Producing new block...");

        let pending_txs = self.mempool.get_transactions(1000);
        let state = self.state.read();
        let current_height = state.current_height();
        drop(state);

        let previous_block = if current_height > 0 {
            self.storage.get_block_by_height(current_height)?
        } else {
            None
        };

        info!("   Height: {} → {}", current_height, current_height + 1);
        info!("   Transactions: {}", pending_txs.len());

        let block = if let Some(prev_block) = previous_block {
            self.consensus.generate_block(
                &prev_block,
                pending_txs.clone(),
                &self.keypair,
            )?
        } else {
            info!("   Creating genesis block");
            let genesis = spirachain_core::create_genesis_block()?;
            genesis
        };

        if let Some(prev) = self.storage.get_latest_block()? {
            self.consensus.validate_block(&block, &prev)?;
        }

        self.storage.store_block(&block)?;

        {
            let mut state = self.state.write();
            for tx in &block.transactions {
                if let Err(e) = state.transfer(&tx.from, &tx.to, tx.amount) {
                    warn!("Failed to transfer in block: {}", e);
                } else {
                    state.increment_nonce(&tx.from);
                }
            }
            state.set_height(block.header.block_height);
        }

        for tx in &pending_txs {
            let hash = tx.hash();
            self.mempool.remove_transaction(&hash);
        }

        self.blocks_produced += 1;
        self.validator.blocks_proposed += 1;
        self.validator.last_block_height = block.header.block_height;

        info!("✅ Block {} produced successfully!", block.header.block_height);
        info!("   Hash: {}", block.hash());
        info!("   Transactions: {}", block.header.tx_count);
        info!("   Complexity: {:.3}", block.spiral.metadata.complexity);

        Ok(())
    }

    pub async fn submit_transaction(&mut self, tx: Transaction) -> Result<()> {
        info!("📥 Received transaction: {} → {} ({} QBT)",
            tx.from.to_string()[..16].to_string(),
            tx.to.to_string()[..16].to_string(),
            tx.amount.value() as f64 / 1e18
        );

        tx.validate()?;

        let state = self.state.read();
        let balance = state.get_balance(&tx.from);
        drop(state);

        let required = Amount::new(tx.amount.value() + tx.fee.value());
        if balance < required {
            return Err(spirachain_core::SpiraChainError::InsufficientBalance);
        }

        self.mempool.add_transaction(tx)?;

        Ok(())
    }

    fn check_mempool(&self) {
        let size = self.mempool.size();
        if size > 0 {
            info!("💾 Mempool: {} pending transactions", size);
        }
    }

    fn print_stats(&self) {
        let height = self.storage.get_chain_height().unwrap_or(0);
        let mempool_size = self.mempool.size();
        let state = self.state.read();

        info!("📊 Validator Stats:");
        info!("   Height: {}", height);
        info!("   Blocks produced: {}", self.blocks_produced);
        info!("   Mempool: {} txs", mempool_size);
        info!("   Accounts: {}", state.account_count());
        info!("   Reputation: {:.2}", self.validator.reputation_score);
    }

    pub fn stop(&self) {
        *self.is_running.write() = false;
        info!("Stopping validator node...");
    }

    pub fn blocks_produced(&self) -> u64 {
        self.blocks_produced
    }

    pub fn validator_address(&self) -> Address {
        self.validator.address
    }

    pub fn reputation_score(&self) -> f64 {
        self.validator.reputation_score
    }
    
    pub fn last_block_height(&self) -> u64 {
        self.validator.last_block_height
    }
}
