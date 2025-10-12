use crate::{Address, Amount, Hash, PiCoordinate, SpiralPosition, EntityType, IntentType, Result, SpiraChainError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub intent_type: IntentType,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u64,
    pub tx_hash: Hash,
    pub pi_id: PiCoordinate,
    
    pub from: Address,
    pub to: Address,
    pub amount: Amount,
    pub fee: Amount,
    pub timestamp: u64,
    pub signature: Vec<u8>,
    
    pub purpose: String,
    pub semantic_vector: Vec<f32>,
    pub entities: Vec<Entity>,
    pub intent: Option<Intent>,
    pub related_txs: Vec<Hash>,
    
    pub spiral_position: Option<SpiralPosition>,
    pub thread_id: Option<Hash>,
    
    pub extra_data: HashMap<String, Vec<u8>>,
}

impl Transaction {
    pub fn new(from: Address, to: Address, amount: Amount, fee: Amount) -> Self {
        Self {
            version: 1,
            tx_hash: Hash::zero(),
            pi_id: PiCoordinate::zero(),
            from,
            to,
            amount,
            fee,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            signature: Vec::new(),
            purpose: String::new(),
            semantic_vector: Vec::new(),
            entities: Vec::new(),
            intent: None,
            related_txs: Vec::new(),
            spiral_position: None,
            thread_id: None,
            extra_data: HashMap::new(),
        }
    }

    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = purpose.into();
        self
    }

    pub fn with_pi_id(mut self, pi_id: PiCoordinate) -> Self {
        self.pi_id = pi_id;
        self
    }

    pub fn with_spiral_position(mut self, position: SpiralPosition) -> Self {
        self.spiral_position = Some(position);
        self
    }

    pub fn with_semantic_vector(mut self, vector: Vec<f32>) -> Self {
        self.semantic_vector = vector;
        self
    }

    pub fn with_intent(mut self, intent: Intent) -> Self {
        self.intent = Some(intent);
        self
    }

    pub fn with_entities(mut self, entities: Vec<Entity>) -> Self {
        self.entities = entities;
        self
    }

    pub fn with_thread_id(mut self, thread_id: Hash) -> Self {
        self.thread_id = Some(thread_id);
        self
    }

    pub fn with_extra_data(mut self, key: impl Into<String>, value: Vec<u8>) -> Self {
        self.extra_data.insert(key.into(), value);
        self
    }

    pub fn compute_hash(&mut self) {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.version.to_be_bytes());
        hasher.update(self.from.as_bytes());
        hasher.update(self.to.as_bytes());
        hasher.update(&self.amount.value().to_be_bytes());
        hasher.update(&self.fee.value().to_be_bytes());
        hasher.update(&self.timestamp.to_be_bytes());
        hasher.update(self.purpose.as_bytes());
        
        for &coord in &[self.pi_id.x, self.pi_id.y, self.pi_id.z, self.pi_id.t] {
            hasher.update(&coord.to_be_bytes());
        }
        
        self.tx_hash = hasher.finalize().into();
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| SpiraChainError::SerializationError(e.to_string()))
    }

    pub fn validate(&self) -> Result<()> {
        if self.amount.value() == 0 {
            return Err(SpiraChainError::InvalidTransaction("Amount cannot be zero".to_string()));
        }

        if self.fee.value() < crate::MIN_TX_FEE {
            return Err(SpiraChainError::InvalidTransaction(
                format!("Fee too low: {} < {}", self.fee, Amount::new(crate::MIN_TX_FEE))
            ));
        }

        if self.signature.is_empty() {
            return Err(SpiraChainError::InvalidSignature);
        }

        if self.from == Address::zero() || self.to == Address::zero() {
            return Err(SpiraChainError::InvalidTransaction("Invalid address".to_string()));
        }

        Ok(())
    }

    pub fn semantic_coherence(&self) -> f64 {
        if self.semantic_vector.is_empty() {
            return 0.0;
        }
        
        let magnitude: f32 = self.semantic_vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude < 0.01 {
            return 0.0;
        }
        
        magnitude.min(1.0) as f64
    }
    
    pub fn hash(&self) -> Hash {
        let data = self.serialize();
        Hash::from(blake3::hash(&data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let from = Address::new([1u8; 32]);
        let to = Address::new([2u8; 32]);
        let amount = Amount::qbt(100);
        let fee = Amount::from_millis(1);
        
        let mut tx = Transaction::new(from, to, amount, fee)
            .with_purpose("Test transaction");
        
        tx.compute_hash();
        
        assert_eq!(tx.from, from);
        assert_eq!(tx.to, to);
        assert_eq!(tx.amount, amount);
        assert_ne!(tx.tx_hash, Hash::zero());
    }

    #[test]
    fn test_transaction_validation() {
        let from = Address::new([1u8; 32]);
        let to = Address::new([2u8; 32]);
        let amount = Amount::qbt(100);
        let fee = Amount::from_millis(1);
        
        let mut tx = Transaction::new(from, to, amount, fee);
        tx.signature = vec![0u8; 64];
        
        assert!(tx.validate().is_ok());
    }

    #[test]
    fn test_invalid_transaction_no_signature() {
        let from = Address::new([1u8; 32]);
        let to = Address::new([2u8; 32]);
        let amount = Amount::qbt(100);
        let fee = Amount::from_millis(1);
        
        let tx = Transaction::new(from, to, amount, fee);
        
        assert!(tx.validate().is_err());
    }
}

