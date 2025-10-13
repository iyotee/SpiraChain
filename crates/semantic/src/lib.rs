pub mod embeddings;
pub mod patterns;
pub mod narrative;
pub mod entities;

pub use embeddings::*;
pub use patterns::*;
pub use narrative::*;
pub use entities::*;

use spirachain_core::{Transaction, Result};
use spirapi_bridge::SpiraPiEngine;
use tracing::warn;

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
        // Tenter d'utiliser SpiraPi AI
        match SpiraPiEngine::generate_embedding(text) {
            Ok(embedding) => {
                if embedding.iter().any(|&v| v != 0.0) {
                    Ok(embedding)
                } else {
                    // Embedding vide, utiliser fallback
                    Ok(self.simple_embedding_fallback(text))
                }
            }
            Err(e) => {
                warn!("SpiraPi embedding failed, using fallback: {}", e);
                Ok(self.simple_embedding_fallback(text))
            }
        }
    }
    
    fn simple_embedding_fallback(&self, text: &str) -> Vec<f32> {
        use blake3::hash;
        
        if text.is_empty() {
            return vec![0.0; 384];
        }
        
        // Générer embedding basique depuis hash
        let hash_bytes = hash(text.as_bytes());
        let mut vec = vec![0.0; 384];
        
        // Distribuer les bytes du hash sur le vecteur
        for (i, byte) in hash_bytes.as_bytes().iter().enumerate() {
            let base_idx = (i * 12) % 384;
            for j in 0..12 {
                if base_idx + j < 384 {
                    vec[base_idx + j] = (*byte as f32) / 255.0;
                }
            }
        }
        
        // Normaliser le vecteur
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in &mut vec {
                *v /= norm;
            }
        }
        
        vec
    }

    fn extract_entities(&self, text: &str) -> Vec<spirachain_core::Entity> {
        let mut entities = Vec::new();
        
        if text.is_empty() {
            return entities;
        }
        
        // Détection d'adresses Ethereum/SpiraChain (0x...)
        let mut search_start = 0;
        while let Some(addr_start) = text[search_start..].find("0x") {
            let abs_start = search_start + addr_start;
            let remaining = &text[abs_start..];
            
            // Extraire jusqu'à 66 caractères (0x + 64 hex chars)
            let end_idx = remaining.chars()
                .take_while(|c| c.is_ascii_hexdigit() || *c == 'x')
                .count();
            
            if end_idx > 2 {  // Au moins "0x" + quelques chars
                let address = &remaining[..end_idx.min(66)];
                entities.push(spirachain_core::Entity {
                    name: address.to_string(),
                    entity_type: spirachain_core::EntityType::Concept, // Addresses as Concepts
                    confidence: if end_idx >= 42 { 0.95 } else { 0.7 }, // 0x + 40 hex = adresse complète
                });
            }
            
            search_start = abs_start + end_idx.max(1);
            if search_start >= text.len() {
                break;
            }
        }
        
        entities
    }

    fn classify_intent(&self, text: &str) -> Option<spirachain_core::Intent> {
        if text.is_empty() {
            return Some(spirachain_core::Intent {
                intent_type: spirachain_core::IntentType::Transfer,
                confidence: 0.5,
            });
        }
        
        let text_lower = text.to_lowercase();
        
        let (intent_type, confidence) = if text_lower.contains("transfer") 
            || text_lower.contains("send") 
            || text_lower.contains("payment")
            || text_lower.contains("pay") {
            (spirachain_core::IntentType::Transfer, 0.9)
        } else if text_lower.contains("contract") 
            || text_lower.contains("deploy")
            || text_lower.contains("execute") {
            (spirachain_core::IntentType::ContractCall, 0.8)
        } else if text_lower.contains("data") 
            || text_lower.contains("store")
            || text_lower.contains("save") {
            (spirachain_core::IntentType::DataStorage, 0.75)
        } else if text_lower.contains("vote") 
            || text_lower.contains("govern")
            || text_lower.contains("proposal") {
            (spirachain_core::IntentType::Governance, 0.7)
        } else if text_lower.contains("social")
            || text_lower.contains("message")
            || text_lower.contains("post") {
            (spirachain_core::IntentType::Social, 0.7)
        } else {
            // Par défaut, supposer transfer avec faible confiance
            (spirachain_core::IntentType::Transfer, 0.5)
        };
        
        Some(spirachain_core::Intent {
            intent_type,
            confidence,
        })
    }

    pub fn calculate_coherence(&self, transactions: &[Transaction]) -> f64 {
        if transactions.is_empty() {
            return 0.0;
        }
        
        // Extraire les embeddings non-vides
        let embeddings: Vec<Vec<f32>> = transactions.iter()
            .filter_map(|tx| {
                if tx.semantic_vector.is_empty() {
                    None
                } else {
                    Some(tx.semantic_vector.clone())
                }
            })
            .collect();
        
        if embeddings.is_empty() {
            return 0.0;
        }
        
        if embeddings.len() == 1 {
            return 1.0; // Une seule transaction = parfaitement cohérente
        }
        
        // Tenter d'utiliser SpiraPi pour calcul optimisé
        match SpiraPiEngine::calculate_coherence(&embeddings) {
            Ok(coherence) => coherence,
            Err(_) => {
                // Fallback: moyenne des scores sémantiques individuels
                let sum: f64 = transactions.iter()
                    .map(|tx| tx.semantic_coherence())
                    .sum();
                sum / (transactions.len() as f64)
            }
        }
    }
}

impl Default for SemanticProcessor {
    fn default() -> Self {
        Self::new("http://localhost:8000".to_string())
    }
}

