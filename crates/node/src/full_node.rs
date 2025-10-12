use spirachain_core::{Block, Transaction, Hash, Result};
use crate::{NodeConfig, Mempool, WorldState, BlockStorage};

pub struct FullNode {
    config: NodeConfig,
    mempool: Mempool,
    state: WorldState,
    storage: BlockStorage,
}

impl FullNode {
    pub fn new(config: NodeConfig) -> Result<Self> {
        let storage = BlockStorage::new(&config.data_dir)?;

        Ok(Self {
            config,
            mempool: Mempool::default(),
            state: WorldState::default(),
            storage,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting full node...");
        tracing::info!("Data directory: {:?}", self.config.data_dir);
        tracing::info!("RPC listening on: {}", self.config.rpc_addr);

        Ok(())
    }

    pub async fn process_block(&mut self, block: Block) -> Result<()> {
        block.validate()?;

        self.storage.store_block(&block)?;

        for tx in &block.transactions {
            self.state.transfer(&tx.from, &tx.to, tx.amount)?;
            self.mempool.remove_transaction(&tx.tx_hash);
        }

        self.state.set_height(block.header.block_height);

        Ok(())
    }

    pub async fn get_block(&self, hash: &Hash) -> Result<Option<Block>> {
        self.storage.get_block(hash)
    }

    pub async fn get_transaction(&self, hash: &Hash) -> Result<Option<Transaction>> {
        self.storage.get_transaction(hash)
    }

    pub fn current_height(&self) -> u64 {
        self.state.current_height()
    }
}

