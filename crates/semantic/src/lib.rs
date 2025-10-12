pub mod embeddings;
pub mod patterns;
pub mod narrative;
pub mod entities;

pub use embeddings::*;
pub use patterns::*;
pub use narrative::*;
pub use entities::*;

use spirachain_core::{Transaction, Result};

pub struct SemanticProcessor {
    embedding_service_url: String,
}

impl SemanticProcessor {
    pub fn new(embedding_service_url: String) -> Self {
        Self {
            embedding_service_url,
        }
    }

    pub async fn enrich_transaction(&self, mut tx: Transaction) -> Result<Transaction> {
        if !tx.purpose.is_empty() {
            let embedding = self.generate_embedding(&tx.purpose).await?;
            tx = tx.with_semantic_vector(embedding);
        }

        let entities = self.extract_entities(&tx.purpose);
        tx = tx.with_entities(entities);

        let intent = self.classify_intent(&tx.purpose);
        if let Some(i) = intent {
            tx = tx.with_intent(i);
        }

        Ok(tx)
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        Ok(vec![0.0; 384])
    }

    fn extract_entities(&self, text: &str) -> Vec<spirachain_core::Entity> {
        Vec::new()
    }

    fn classify_intent(&self, text: &str) -> Option<spirachain_core::Intent> {
        Some(spirachain_core::Intent {
            intent_type: spirachain_core::IntentType::Transfer,
            confidence: 0.8,
        })
    }

    pub fn calculate_coherence(&self, transactions: &[Transaction]) -> f64 {
        if transactions.is_empty() {
            return 0.0;
        }

        let sum: f64 = transactions.iter()
            .map(|tx| tx.semantic_coherence())
            .sum();

        sum / (transactions.len() as f64)
    }
}

impl Default for SemanticProcessor {
    fn default() -> Self {
        Self::new("http://localhost:8000".to_string())
    }
}

