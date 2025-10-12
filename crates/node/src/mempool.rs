use spirachain_core::{Transaction, Hash, Result};
use std::collections::HashMap;

pub struct Mempool {
    transactions: HashMap<Hash, Transaction>,
    max_size: usize,
}

impl Mempool {
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            max_size,
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> Result<()> {
        if self.transactions.len() >= self.max_size {
            self.evict_lowest_fee();
        }

        self.transactions.insert(tx.tx_hash, tx);
        Ok(())
    }

    pub fn remove_transaction(&mut self, hash: &Hash) -> Option<Transaction> {
        self.transactions.remove(hash)
    }

    pub fn get_transaction(&self, hash: &Hash) -> Option<&Transaction> {
        self.transactions.get(hash)
    }

    pub fn get_pending_transactions(&self, limit: usize) -> Vec<Transaction> {
        let mut txs: Vec<Transaction> = self.transactions.values().cloned().collect();
        txs.sort_by(|a, b| b.fee.cmp(&a.fee));
        txs.into_iter().take(limit).collect()
    }

    pub fn size(&self) -> usize {
        self.transactions.len()
    }

    pub fn clear(&mut self) {
        self.transactions.clear();
    }

    fn evict_lowest_fee(&mut self) {
        if let Some((&hash, _)) = self.transactions.iter()
            .min_by_key(|(_, tx)| tx.fee) {
            self.transactions.remove(&hash);
        }
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new(10000)
    }
}

