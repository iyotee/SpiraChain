use spirachain_core::{Transaction, Hash, Result, SpiraChainError};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Clone)]
pub struct Mempool {
    transactions: Arc<RwLock<HashMap<Hash, Transaction>>>,
    pending_queue: Arc<RwLock<VecDeque<Hash>>>,
    max_size: usize,
}

impl Mempool {
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
            pending_queue: Arc::new(RwLock::new(VecDeque::new())),
            max_size,
        }
    }

    pub fn add_transaction(&self, tx: Transaction) -> Result<()> {
        let tx_hash = tx.hash();
        
        let mut txs = self.transactions.write();
        let mut queue = self.pending_queue.write();
        
        if txs.len() >= self.max_size {
            return Err(SpiraChainError::Internal("Mempool full".to_string()));
        }

        if txs.contains_key(&tx_hash) {
            return Err(SpiraChainError::InvalidTransaction("Transaction already in mempool".to_string()));
        }

        txs.insert(tx_hash, tx);
        queue.push_back(tx_hash);
        
        tracing::info!("Added transaction {} to mempool", hex::encode(tx_hash.as_bytes()));
        
        Ok(())
    }

    pub fn get_pending_transactions(&self, max_count: usize) -> Vec<Transaction> {
        let txs = self.transactions.read();
        let queue = self.pending_queue.read();
        
        queue.iter()
            .take(max_count)
            .filter_map(|hash| txs.get(hash).cloned())
            .collect()
    }

    pub fn remove_transactions(&self, tx_hashes: &[Hash]) {
        let mut txs = self.transactions.write();
        let mut queue = self.pending_queue.write();
        
        for hash in tx_hashes {
            txs.remove(hash);
            queue.retain(|h| h != hash);
        }
        
        tracing::info!("Removed {} transactions from mempool", tx_hashes.len());
    }
    
    pub fn remove_transaction(&self, tx_hash: &Hash) {
        self.remove_transactions(&[*tx_hash]);
    }

    pub fn get_transaction(&self, hash: &Hash) -> Option<Transaction> {
        self.transactions.read().get(hash).cloned()
    }

    pub fn size(&self) -> usize {
        self.transactions.read().len()
    }

    pub fn clear(&self) {
        self.transactions.write().clear();
        self.pending_queue.write().clear();
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new(10000)
    }
}
