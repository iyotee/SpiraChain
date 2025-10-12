use spirachain_core::{Block, Transaction, Result, SpiraChainError, Hash};
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
}

impl ValidatorNode {
    pub fn new(config: NodeConfig, keypair: KeyPair, validator: Validator) -> Result<Self> {
        let storage = BlockStorage::new(&config.data_dir)?;
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
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 Starting SpiraChain Validator Node");
        info!("   Address: {}", self.keypair.to_address());
        info!("   Stake: {}", self.validator.stake);
        info!("   Data dir: {}", &self.config.data_dir);

        *self.is_running.write() = true;

        let genesis_block = self.initialize_genesis().await?;
        info!("   Genesis block hash: {}", hex::encode(genesis_block.hash().as_bytes()));

        self.run_validation_loop().await?;

        Ok(())
    }

    async fn initialize_genesis(&mut self) -> Result<Block> {
        let latest = self.storage.get_latest_block()?;
        
        if latest.is_none() {
            info!("Creating genesis block...");
            
            let genesis_config = spirachain_core::GenesisConfig::default();
            let genesis_block = spirachain_core::create_genesis_block(&genesis_config);
            
            self.storage.store_block(&genesis_block)?;
            
            for allocation in &genesis_config.genesis_transactions {
                let mut state = self.state.write();
                state.set_balance(allocation.recipient, allocation.amount);
            }
            
            info!("✅ Genesis block created");
            Ok(genesis_block)
        } else {
            info!("Genesis block already exists");
            Ok(latest.unwrap())
        }
    }

    async fn run_validation_loop(&mut self) -> Result<()> {
        let mut block_interval = interval(Duration::from_secs(60));
        let mut stats_interval = interval(Duration::from_secs(10));

        info!("⚡ Validation loop started (60s block time)");

        loop {
            tokio::select! {
                _ = block_interval.tick() => {
                    if let Err(e) = self.try_produce_block().await {
                        error!("Failed to produce block: {}", e);
                    }
                }
                
                _ = stats_interval.tick() => {
                    self.print_stats();
                }
            }

            if !*self.is_running.read() {
                info!("Validator node stopped");
                break;
            }
        }

        Ok(())
    }

    async fn try_produce_block(&mut self) -> Result<()> {
        let pending_txs = self.mempool.get_pending_transactions(100);
        
        if pending_txs.is_empty() {
            info!("No transactions in mempool, skipping block production");
            return Ok(());
        }

        info!("🔨 Producing block with {} transactions", pending_txs.len());

        let previous_block = self.storage.get_latest_block()?
            .ok_or_else(|| SpiraChainError::BlockNotFound("No previous block".to_string()))?;

        let block = self.consensus.generate_block_candidate(
            &self.validator,
            &self.keypair,
            pending_txs.clone(),
            &previous_block,
        )?;

        if self.consensus.validate_block(&block, &previous_block).is_ok() {
            self.storage.store_block(&block)?;
            
            let tx_hashes: Vec<Hash> = pending_txs.iter().map(|tx| tx.hash()).collect();
            self.mempool.remove_transactions(&tx_hashes);

            self.apply_block_to_state(&block)?;

            info!("✅ Block {} produced at height {}", 
                hex::encode(block.hash().as_bytes())[..16].to_string(),
                block.header.block_height
            );
        } else {
            warn!("Block validation failed");
        }

        Ok(())
    }

    fn apply_block_to_state(&mut self, block: &Block) -> Result<()> {
        let mut state = self.state.write();
        
        for tx in &block.transactions {
            state.transfer(&tx.from, &tx.to, tx.amount)?;
            state.increment_nonce(&tx.from);
        }
        
        state.set_height(block.header.block_height);
        
        Ok(())
    }

    pub async fn submit_transaction(&mut self, tx: Transaction) -> Result<()> {
        info!("Received transaction: {} → {} ({} QBT)", 
            tx.from.to_string()[..8].to_string(),
            tx.to.to_string()[..8].to_string(),
            tx.amount.to_string()
        );

        self.mempool.add_transaction(tx)?;
        
        Ok(())
    }

    pub fn get_address(&self) -> spirachain_core::Address {
        self.keypair.to_address()
    }

    pub fn stop(&self) {
        *self.is_running.write() = false;
        info!("Stopping validator node...");
    }

    fn print_stats(&self) {
        let height = self.storage.get_chain_height().unwrap_or(0);
        let mempool_size = self.mempool.size();
        let state = self.state.read();
        
        info!("📊 Stats: Height={} Mempool={} State_Accounts={}", 
            height, 
            mempool_size,
            state.account_count()
        );
    }
}

// NodeConfig moved to lib.rs to avoid duplication
