use parking_lot::RwLock;
use spirachain_core::{Hash, Result, SpiraChainError, Transaction};
use spirachain_semantic::SemanticProcessor;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

#[derive(Clone)]
pub struct Mempool {
    transactions: Arc<RwLock<HashMap<Hash, Transaction>>>,
    pending_queue: Arc<RwLock<VecDeque<Hash>>>,
    max_size: usize,
    semantic_processor: Arc<SemanticProcessor>,
}

impl Mempool {
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
            pending_queue: Arc::new(RwLock::new(VecDeque::new())),
            max_size,
            semantic_processor: Arc::new(SemanticProcessor::default()),
        }
    }

    pub async fn add_transaction(&self, mut tx: Transaction) -> Result<()> {
        // Enrichissement sémantique si purpose présent
        if !tx.purpose.is_empty() {
            match self.semantic_processor.enrich_transaction(tx.clone()).await {
                Ok(enriched_tx) => {
                    tracing::debug!("Transaction enriched with semantic data");
                    tx = enriched_tx;
                }
                Err(e) => {
                    tracing::warn!("Failed to enrich transaction semantically: {}", e);
                    // Continue avec la transaction non-enrichie
                }
            }
        }

        let tx_hash = tx.hash();

        let mut txs = self.transactions.write();
        let mut queue = self.pending_queue.write();

        if txs.len() >= self.max_size {
            return Err(SpiraChainError::Internal("Mempool full".to_string()));
        }

        if txs.contains_key(&tx_hash) {
            return Err(SpiraChainError::InvalidTransaction(
                "Transaction already in mempool".to_string(),
            ));
        }

        txs.insert(tx_hash, tx);
        queue.push_back(tx_hash);

        tracing::info!(
            "Added transaction {} to mempool",
            hex::encode(tx_hash.as_bytes())
        );

        Ok(())
    }

    pub fn add_transaction_sync(&self, tx: Transaction) -> Result<()> {
        // Version synchrone sans enrichissement pour compatibilité
        let tx_hash = tx.hash();

        let mut txs = self.transactions.write();
        let mut queue = self.pending_queue.write();

        if txs.len() >= self.max_size {
            return Err(SpiraChainError::Internal("Mempool full".to_string()));
        }

        if txs.contains_key(&tx_hash) {
            return Err(SpiraChainError::InvalidTransaction(
                "Transaction already in mempool".to_string(),
            ));
        }

        txs.insert(tx_hash, tx);
        queue.push_back(tx_hash);

        tracing::info!(
            "Added transaction {} to mempool (sync)",
            hex::encode(tx_hash.as_bytes())
        );

        Ok(())
    }

    pub fn get_pending_transactions(&self, max_count: usize) -> Vec<Transaction> {
        let txs = self.transactions.read();
        let queue = self.pending_queue.read();

        queue
            .iter()
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

    pub fn get_all_transactions(&self) -> Vec<Transaction> {
        self.transactions.read().values().cloned().collect()
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
