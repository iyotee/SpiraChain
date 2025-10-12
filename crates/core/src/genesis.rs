use crate::{Block, BlockHeader, Transaction, Hash, PiCoordinate, SpiralMetadata, SpiralType, Address, Amount, Entity, Intent, EntityType, IntentType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {
    pub version: u64,
    pub timestamp: u64,
    pub manifesto: String,
    pub founding_principles: Vec<String>,
    pub initial_validators: Vec<GenesisValidator>,
    pub genesis_transactions: Vec<GenesisAllocation>,
    pub constants: GenesisConstants,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisValidator {
    pub name: String,
    pub pubkey: Vec<u8>,
    pub geographic_region: String,
    pub stake: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisAllocation {
    pub recipient: Address,
    pub amount: u128,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConstants {
    pub pi_precision: usize,
    pub e_precision: usize,
    pub phi_precision: usize,
    pub block_time_target: u64,
    pub max_block_size: usize,
    pub semantic_dimensions: usize,
    pub min_validator_stake: u128,
    pub qubitum_decimals: u8,
}

impl Default for GenesisConfig {
    fn default() -> Self {
        Self {
            version: 1,
            timestamp: 1737331200000,
            manifesto: include_str!("../../manifesto.txt").to_string(),
            founding_principles: vec![
                "Mathematical beauty as consensus".to_string(),
                "Semantic coherence over computational waste".to_string(),
                "Post-quantum security from inception".to_string(),
                "Adaptive intelligence through native AI".to_string(),
                "Geometric truth over hierarchical control".to_string(),
            ],
            initial_validators: Self::create_initial_validators(),
            genesis_transactions: Self::create_genesis_allocations(),
            constants: GenesisConstants {
                pi_precision: crate::PI_PRECISION,
                e_precision: crate::E_PRECISION,
                phi_precision: crate::PHI_PRECISION,
                block_time_target: crate::BLOCK_TIME_TARGET,
                max_block_size: crate::MAX_BLOCK_SIZE,
                semantic_dimensions: crate::SEMANTIC_VECTOR_DIM,
                min_validator_stake: crate::MIN_VALIDATOR_STAKE,
                qubitum_decimals: crate::TOKEN_DECIMALS,
            },
        }
    }
}

impl GenesisConfig {
    fn create_initial_validators() -> Vec<GenesisValidator> {
        vec![
            GenesisValidator {
                name: "Archimedes Node".to_string(),
                pubkey: vec![0u8; 32],
                geographic_region: "Europe".to_string(),
                stake: 50_000 * 10u128.pow(18),
            },
            GenesisValidator {
                name: "Ramanujan Node".to_string(),
                pubkey: vec![1u8; 32],
                geographic_region: "Asia".to_string(),
                stake: 50_000 * 10u128.pow(18),
            },
            GenesisValidator {
                name: "Fibonacci Node".to_string(),
                pubkey: vec![2u8; 32],
                geographic_region: "North America".to_string(),
                stake: 50_000 * 10u128.pow(18),
            },
            GenesisValidator {
                name: "Euclid Node".to_string(),
                pubkey: vec![3u8; 32],
                geographic_region: "South America".to_string(),
                stake: 50_000 * 10u128.pow(18),
            },
            GenesisValidator {
                name: "Pythagoras Node".to_string(),
                pubkey: vec![4u8; 32],
                geographic_region: "Africa".to_string(),
                stake: 50_000 * 10u128.pow(18),
            },
        ]
    }

    fn create_genesis_allocations() -> Vec<GenesisAllocation> {
        let total_supply = crate::INITIAL_SUPPLY;
        
        vec![
            GenesisAllocation {
                recipient: Address::new([1u8; 32]),
                amount: (total_supply as f64 * 0.30) as u128,
                purpose: "Team & development fund - 4 year vesting".to_string(),
            },
            GenesisAllocation {
                recipient: Address::new([2u8; 32]),
                amount: (total_supply as f64 * 0.20) as u128,
                purpose: "Early validator rewards".to_string(),
            },
            GenesisAllocation {
                recipient: Address::new([3u8; 32]),
                amount: (total_supply as f64 * 0.15) as u128,
                purpose: "Research grants".to_string(),
            },
            GenesisAllocation {
                recipient: Address::new([4u8; 32]),
                amount: (total_supply as f64 * 0.10) as u128,
                purpose: "Community treasury (DAO-controlled)".to_string(),
            },
            GenesisAllocation {
                recipient: Address::new([5u8; 32]),
                amount: (total_supply as f64 * 0.10) as u128,
                purpose: "Liquidity provisions".to_string(),
            },
            GenesisAllocation {
                recipient: Address::new([6u8; 32]),
                amount: (total_supply as f64 * 0.15) as u128,
                purpose: "Public genesis auction".to_string(),
            },
        ]
    }

    pub fn create_genesis_block(&self) -> Block {
        let mut genesis_block = Block::new(Hash::zero(), 0);
        
        genesis_block.header.timestamp = self.timestamp;
        genesis_block.header.version = self.version;
        
        let genesis_spiral = SpiralMetadata {
            spiral_type: SpiralType::Ramanujan,
            complexity: 100.0,
            self_similarity: 1.618,
            information_density: 3.14159,
            semantic_coherence: 1.0,
            geometry_data: vec![],
        };
        
        genesis_block.header.spiral = genesis_spiral;
        
        genesis_block.header.pi_coordinates = PiCoordinate::new(
            3.141592653589793,
            2.718281828459045,
            1.618033988749895,
            0.0,
        );
        
        let mut transactions = Vec::new();
        
        for allocation in &self.genesis_transactions {
            let mut tx = Transaction::new(
                Address::zero(),
                allocation.recipient,
                Amount::new(allocation.amount),
                Amount::zero(),
            );
            
            tx.purpose = allocation.purpose.clone();
            tx.timestamp = self.timestamp;
            tx.intent = Some(Intent {
                intent_type: IntentType::Transfer,
                confidence: 1.0,
            });
            
            tx.compute_hash();
            transactions.push(tx);
        }
        
        genesis_block = genesis_block.with_transactions(transactions);
        genesis_block.compute_merkle_root();
        genesis_block.compute_spiral_root();
        
        genesis_block.header.signature = vec![0u8; 64];
        
        genesis_block
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_config_creation() {
        let config = GenesisConfig::default();
        assert_eq!(config.version, 1);
        assert_eq!(config.initial_validators.len(), 5);
        assert_eq!(config.genesis_transactions.len(), 6);
    }

    #[test]
    fn test_genesis_block_creation() {
        let config = GenesisConfig::default();
        let genesis_block = config.create_genesis_block();
        
        assert_eq!(genesis_block.header.block_height, 0);
        assert_eq!(genesis_block.header.previous_block_hash, Hash::zero());
        assert!(genesis_block.is_genesis());
        assert!(!genesis_block.transactions.is_empty());
    }

    #[test]
    fn test_total_allocation() {
        let config = GenesisConfig::default();
        let total: u128 = config.genesis_transactions.iter()
            .map(|alloc| alloc.amount)
            .sum();
        
        assert_eq!(total, crate::INITIAL_SUPPLY);
    }

    #[test]
    fn test_genesis_config_serialization() {
        let config = GenesisConfig::default();
        let json = config.to_json();
        assert!(!json.is_empty());
        
        let deserialized = GenesisConfig::from_json(&json).unwrap();
        assert_eq!(deserialized.version, config.version);
    }
}

pub fn create_genesis_block(config: &GenesisConfig) -> Block {
    let mut block = Block::new(Hash::zero(), 0);
    block.header.timestamp = config.timestamp;
    block.header.version = config.version;
    block
}

