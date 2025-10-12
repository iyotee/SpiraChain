use spirachain_core::{Block, Transaction, Result};
use spirachain_crypto::KeyPair;
use spirachain_consensus::{ProofOfSpiral, Validator};
use crate::{NodeConfig, Mempool, WorldState, BlockStorage};

pub struct ValidatorNode {
    config: NodeConfig,
    keypair: KeyPair,
    validator: Validator,
    mempool: Mempool,
    state: WorldState,
    storage: BlockStorage,
    consensus: ProofOfSpiral,
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
            state: WorldState::default(),
            storage,
            consensus,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting validator node...");
        tracing::info!("Address: {}", self.keypair.to_address());
        tracing::info!("Stake: {}", self.validator.stake);

        Ok(())
    }

    pub async fn generate_block(&mut self, previous_block: &Block) -> Result<Block> {
        let pending_txs = self.mempool.get_pending_transactions(100);
        
        let block = self.consensus.generate_block_candidate(
            &self.validator,
            &self.keypair,
            pending_txs,
            previous_block
        )?;

        Ok(block)
    }

    pub async fn submit_transaction(&mut self, tx: Transaction) -> Result<()> {
        self.mempool.add_transaction(tx)
    }

    pub fn get_address(&self) -> spirachain_core::Address {
        self.keypair.to_address()
    }
}

