use sled::{Db, Tree};
use spirachain_core::{Address, Amount, Block, Hash, Result, SpiraChainError, Transaction};
use std::path::Path;

pub struct NodeStorage {
    db: Db,
    blocks: Tree,
    transactions: Tree,
    state: Tree,
    block_by_height: Tree,
}

impl NodeStorage {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db = sled::open(path.as_ref()).map_err(|e| {
            SpiraChainError::StorageError(format!("Failed to open database: {}", e))
        })?;

        let blocks = db.open_tree(b"blocks").map_err(|e| {
            SpiraChainError::StorageError(format!("Failed to open blocks tree: {}", e))
        })?;

        let transactions = db.open_tree(b"transactions").map_err(|e| {
            SpiraChainError::StorageError(format!("Failed to open transactions tree: {}", e))
        })?;

        let state = db.open_tree(b"state").map_err(|e| {
            SpiraChainError::StorageError(format!("Failed to open state tree: {}", e))
        })?;

        let block_by_height = db.open_tree(b"block_by_height").map_err(|e| {
            SpiraChainError::StorageError(format!("Failed to open block_by_height tree: {}", e))
        })?;

        Ok(Self {
            db,
            blocks,
            transactions,
            state,
            block_by_height,
        })
    }

    pub fn store_block(&self, block: &Block) -> Result<()> {
        let block_hash = block.hash();
        let serialized = bincode::serialize(block).map_err(|e| {
            SpiraChainError::SerializationError(format!("Failed to serialize block: {}", e))
        })?;

        self.blocks
            .insert(block_hash.as_bytes(), serialized.as_slice())
            .map_err(|e| SpiraChainError::StorageError(format!("Failed to store block: {}", e)))?;

        let height_key = block.header.block_height.to_be_bytes();
        self.block_by_height
            .insert(height_key, block_hash.as_bytes())
            .map_err(|e| {
                SpiraChainError::StorageError(format!("Failed to index block by height: {}", e))
            })?;

        tracing::info!("Stored block at height {}", block.header.block_height);
        Ok(())
    }

    pub fn get_block(&self, hash: &Hash) -> Result<Option<Block>> {
        match self
            .blocks
            .get(hash.as_bytes())
            .map_err(|e| SpiraChainError::StorageError(format!("Failed to get block: {}", e)))?
        {
            Some(data) => {
                let block: Block = bincode::deserialize(&data).map_err(|e| {
                    SpiraChainError::SerializationError(format!(
                        "Failed to deserialize block: {}",
                        e
                    ))
                })?;
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }

    pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>> {
        let height_key = height.to_be_bytes();

        match self.block_by_height.get(height_key).map_err(|e| {
            SpiraChainError::StorageError(format!("Failed to get block hash by height: {}", e))
        })? {
            Some(hash_bytes) => {
                let mut hash_array = [0u8; 32];
                hash_array.copy_from_slice(&hash_bytes);
                self.get_block(&Hash::from(hash_array))
            }
            None => Ok(None),
        }
    }

    pub fn get_latest_block(&self) -> Result<Option<Block>> {
        let last_entry = self.block_by_height.last().map_err(|e| {
            SpiraChainError::StorageError(format!("Failed to get latest block: {}", e))
        })?;

        match last_entry {
            Some((_, hash_bytes)) => {
                let mut hash_array = [0u8; 32];
                hash_array.copy_from_slice(&hash_bytes);
                self.get_block(&Hash::from(hash_array))
            }
            None => Ok(None),
        }
    }

    pub fn get_chain_height(&self) -> Result<u64> {
        match self.block_by_height.last().map_err(|e| {
            SpiraChainError::StorageError(format!("Failed to get chain height: {}", e))
        })? {
            Some((height_bytes, _)) => {
                let mut height_array = [0u8; 8];
                height_array.copy_from_slice(&height_bytes);
                Ok(u64::from_be_bytes(height_array))
            }
            None => Ok(0),
        }
    }

    pub fn store_transaction(&self, tx: &Transaction) -> Result<()> {
        let tx_hash = tx.hash();
        let serialized = bincode::serialize(tx).map_err(|e| {
            SpiraChainError::SerializationError(format!("Failed to serialize transaction: {}", e))
        })?;

        self.transactions
            .insert(tx_hash.as_bytes(), serialized.as_slice())
            .map_err(|e| {
                SpiraChainError::StorageError(format!("Failed to store transaction: {}", e))
            })?;

        Ok(())
    }

    pub fn get_transaction(&self, hash: &Hash) -> Result<Option<Transaction>> {
        match self.transactions.get(hash.as_bytes()).map_err(|e| {
            SpiraChainError::StorageError(format!("Failed to get transaction: {}", e))
        })? {
            Some(data) => {
                let tx: Transaction = bincode::deserialize(&data).map_err(|e| {
                    SpiraChainError::SerializationError(format!(
                        "Failed to deserialize transaction: {}",
                        e
                    ))
                })?;
                Ok(Some(tx))
            }
            None => Ok(None),
        }
    }

    pub fn store_balance(&self, address: &Address, balance: Amount) -> Result<()> {
        let key = format!("balance:{}", address);
        let value = bincode::serialize(&balance)
            .map_err(|e| SpiraChainError::SerializationError(e.to_string()))?;

        self.state
            .insert(key.as_bytes(), value.as_slice())
            .map_err(|e| SpiraChainError::StorageError(e.to_string()))?;

        Ok(())
    }

    pub fn get_balance(&self, address: &Address) -> Result<Amount> {
        let key = format!("balance:{}", address);

        match self
            .state
            .get(key.as_bytes())
            .map_err(|e| SpiraChainError::StorageError(e.to_string()))?
        {
            Some(data) => {
                let balance: Amount = bincode::deserialize(&data)
                    .map_err(|e| SpiraChainError::SerializationError(e.to_string()))?;
                Ok(balance)
            }
            None => Ok(Amount::zero()),
        }
    }

    pub fn flush(&self) -> Result<()> {
        self.db.flush().map_err(|e| {
            SpiraChainError::StorageError(format!("Failed to flush database: {}", e))
        })?;
        Ok(())
    }
}

pub struct BlockStorage {
    storage: NodeStorage,
}

impl BlockStorage {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            storage: NodeStorage::new(path)?,
        })
    }

    pub fn store_block(&self, block: &Block) -> Result<()> {
        self.storage.store_block(block)
    }

    pub fn get_block(&self, hash: &Hash) -> Result<Option<Block>> {
        self.storage.get_block(hash)
    }

    pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>> {
        self.storage.get_block_by_height(height)
    }

    pub fn get_latest_block(&self) -> Result<Option<Block>> {
        self.storage.get_latest_block()
    }

    pub fn get_chain_height(&self) -> Result<u64> {
        self.storage.get_chain_height()
    }

    pub fn get_transaction(&self, hash: &Hash) -> Result<Option<Transaction>> {
        self.storage.get_transaction(hash)
    }
}
