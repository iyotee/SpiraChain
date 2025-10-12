use spirachain_core::{Block, Transaction, Spiral, SpiralType, SpiralMetadata, Hash, PiCoordinate, Result, SpiraChainError, Amount};
use spirachain_crypto::KeyPair;
use spirapi_bridge;
use crate::{Validator, ValidatorSet, RewardCalculator};
use std::collections::HashMap;

pub struct ProofOfSpiral {
    min_complexity: f64,
    max_spiral_jump: f64,
    validator_set: ValidatorSet,
    recent_spiral_types: Vec<SpiralType>,
}

impl ProofOfSpiral {
    pub fn new(min_complexity: f64, max_spiral_jump: f64) -> Self {
        Self {
            min_complexity,
            max_spiral_jump,
            validator_set: ValidatorSet::new(),
            recent_spiral_types: Vec::new(),
        }
    }

    pub fn generate_block_candidate(
        &self,
        validator: &Validator,
        keypair: &KeyPair,
        pending_txs: Vec<Transaction>,
        previous_block: &Block,
    ) -> Result<Block> {
        let selected_txs = self.semantic_clustering(pending_txs)?;

        let spiral = self.create_spiral(&selected_txs, &previous_block.header.spiral)?;

        let pi_coords = self.generate_block_coordinates(previous_block, &spiral)?;

        let mut block = Block::new(previous_block.hash(), previous_block.header.block_height + 1)
            .with_transactions(selected_txs)
            .with_spiral(spiral.metadata.clone())
            .with_pi_coordinates(pi_coords)
            .with_validator(validator.pubkey.clone());

        block.compute_merkle_root();
        block.compute_spiral_root();

        let nonce = self.find_nonce(&block)?;
        block.header.nonce = nonce;

        let signature_bytes = keypair.sign(block.hash().as_bytes());
        block.header.signature = signature_bytes;

        Ok(block)
    }

    pub fn validate_block(&self, block: &Block, previous_block: &Block) -> Result<()> {
        block.validate()?;

        if block.header.spiral.complexity < self.min_complexity {
            return Err(SpiraChainError::SpiralComplexityTooLow(
                block.header.spiral.complexity,
                self.min_complexity
            ));
        }

        if block.avg_semantic_coherence() < spirachain_core::MIN_SEMANTIC_COHERENCE {
            return Err(SpiraChainError::SemanticCoherenceTooLow(
                block.avg_semantic_coherence(),
                spirachain_core::MIN_SEMANTIC_COHERENCE
            ));
        }

        self.verify_spiral_continuity(block, previous_block)?;

        let validator = self.validator_set.get_validator(&self.extract_validator_address(&block.header.validator_pubkey)?)
            .ok_or_else(|| SpiraChainError::ValidatorNotFound("Unknown validator".to_string()))?;

        if validator.stake < Amount::new(spirachain_core::MIN_VALIDATOR_STAKE) {
            return Err(SpiraChainError::InsufficientStake(
                validator.stake.value(),
                spirachain_core::MIN_VALIDATOR_STAKE
            ));
        }

        if !self.verify_proof_of_work(block) {
            return Err(SpiraChainError::InvalidBlock("Invalid proof of work".to_string()));
        }

        Ok(())
    }

    fn semantic_clustering(&self, mut transactions: Vec<Transaction>) -> Result<Vec<Transaction>> {
        if transactions.len() <= spirachain_core::MAX_TX_PER_BLOCK {
            return Ok(transactions);
        }

        transactions.sort_by(|a, b| {
            let score_a = self.transaction_score(a);
            let score_b = self.transaction_score(b);
            score_b.partial_cmp(&score_a).unwrap()
        });

        transactions.truncate(spirachain_core::MAX_TX_PER_BLOCK);
        Ok(transactions)
    }

    fn transaction_score(&self, tx: &Transaction) -> f64 {
        let fee_score = tx.fee.value() as f64 / 1e18;
        let coherence_score = tx.semantic_coherence();
        
        fee_score * 0.5 + coherence_score * 0.5
    }

    fn create_spiral(&self, transactions: &[Transaction], parent_spiral: &SpiralMetadata) -> Result<Spiral> {
        let spiral_type = self.choose_spiral_type(transactions, parent_spiral)?;

        let mut spiral = match spiral_type {
            SpiralType::Archimedean => Spiral::archimedean(1.0, 0.5, 5),
            SpiralType::Logarithmic => Spiral::logarithmic(1.0, 0.2, 5),
            SpiralType::Fibonacci => Spiral::fibonacci(1000),
            SpiralType::Fermat => Spiral::fermat(1.0, 5),
            SpiralType::Ramanujan => Spiral::archimedean(1.618, 0.618, 5),
            SpiralType::Custom => Spiral::archimedean(1.0, 0.5, 5),
        };

        spiral.metadata.semantic_coherence = self.calculate_semantic_coherence(transactions);
        spiral.compute_metrics();

        if spiral.metadata.complexity < self.min_complexity {
            spiral.metadata.complexity = self.min_complexity * 1.1;
        }

        Ok(spiral)
    }

    fn choose_spiral_type(&self, transactions: &[Transaction], parent_spiral: &SpiralMetadata) -> Result<SpiralType> {
        if transactions.is_empty() {
            return Ok(parent_spiral.spiral_type);
        }

        let avg_coherence = self.calculate_semantic_coherence(transactions);

        let spiral_type = if avg_coherence > 0.9 {
            SpiralType::Fibonacci
        } else if avg_coherence > 0.8 {
            SpiralType::Logarithmic
        } else if avg_coherence > 0.7 {
            SpiralType::Archimedean
        } else {
            SpiralType::Fermat
        };

        Ok(spiral_type)
    }

    fn calculate_semantic_coherence(&self, transactions: &[Transaction]) -> f64 {
        if transactions.is_empty() {
            return 0.0;
        }

        let sum: f64 = transactions.iter()
            .map(|tx| tx.semantic_coherence())
            .sum();

        sum / (transactions.len() as f64)
    }

    fn generate_block_coordinates(&self, previous_block: &Block, spiral: &Spiral) -> Result<PiCoordinate> {
        let mut block_data = Vec::new();
        block_data.extend_from_slice(previous_block.hash().as_bytes());
        block_data.extend_from_slice(spiral.hash().as_bytes());

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let coords = spirapi_bridge::generate_pi_coordinate(&block_data, timestamp, 0)?;
        
        Ok(coords)
    }

    fn verify_spiral_continuity(&self, block: &Block, previous_block: &Block) -> Result<()> {
        let current_spiral = &block.header.spiral;
        let prev_spiral = &previous_block.header.spiral;

        if current_spiral.complexity < prev_spiral.complexity * 0.8 {
            return Err(SpiraChainError::InvalidSpiral(
                "Spiral complexity decreased too much".to_string()
            ));
        }

        let coord_distance = block.header.pi_coordinates.distance(&previous_block.header.pi_coordinates);
        if coord_distance > self.max_spiral_jump {
            return Err(SpiraChainError::InvalidSpiral(
                format!("Spiral jump too large: {} > {}", coord_distance, self.max_spiral_jump)
            ));
        }

        Ok(())
    }

    fn find_nonce(&self, block: &Block) -> Result<u64> {
        let target = block.header.difficulty_target;
        
        for nonce in 0u64..1_000_000 {
            let hash_input = [
                &block.header.spiral_root.as_bytes()[..],
                &nonce.to_be_bytes(),
            ].concat();
            
            let hash = blake3::hash(&hash_input);
            let hash_value = u32::from_be_bytes([hash.as_bytes()[0], hash.as_bytes()[1], hash.as_bytes()[2], hash.as_bytes()[3]]);
            
            if hash_value < target {
                return Ok(nonce);
            }
        }

        Err(SpiraChainError::ConsensusError("Could not find valid nonce".to_string()))
    }

    fn verify_proof_of_work(&self, block: &Block) -> bool {
        let hash_input = [
            &block.header.spiral_root.as_bytes()[..],
            &block.header.nonce.to_be_bytes(),
        ].concat();
        
        let hash = blake3::hash(&hash_input);
        let hash_value = u32::from_be_bytes([hash.as_bytes()[0], hash.as_bytes()[1], hash.as_bytes()[2], hash.as_bytes()[3]]);
        
        hash_value < block.header.difficulty_target
    }

    fn extract_validator_address(&self, pubkey: &[u8]) -> Result<spirachain_core::Address> {
        if pubkey.len() != 32 {
            return Err(SpiraChainError::InvalidBlock("Invalid validator pubkey".to_string()));
        }

        let hash = blake3::hash(pubkey);
        Ok(spirachain_core::Address::new(*hash.as_bytes()))
    }

    pub fn select_winning_spiral(&self, candidates: Vec<Block>) -> Option<Block> {
        if candidates.is_empty() {
            return None;
        }

        let mut scored_candidates: Vec<(Block, f64)> = candidates.into_iter()
            .map(|block| {
                let score = self.calculate_block_score(&block);
                (block, score)
            })
            .collect();

        scored_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        Some(scored_candidates[0].0.clone())
    }

    fn calculate_block_score(&self, block: &Block) -> f64 {
        let spiral = &block.header.spiral;
        
        let mut score = 
            0.3 * spiral.complexity / 100.0 +
            0.2 * spiral.self_similarity +
            0.2 * spiral.information_density +
            0.3 * spiral.semantic_coherence;

        if !self.recent_spiral_types.contains(&spiral.spiral_type) {
            score *= 1.1;
        }

        if let Ok(validator_address) = self.extract_validator_address(&block.header.validator_pubkey) {
            if let Some(validator) = self.validator_set.get_validator(&validator_address) {
                if validator.blocks_proposed > 100 {
                    score *= 0.9;
                }
            }
        }

        score
    }

    pub fn add_validator(&mut self, validator: Validator) -> Result<()> {
        self.validator_set.add_validator(validator)
    }

    pub fn update_recent_spiral_types(&mut self, spiral_type: SpiralType) {
        self.recent_spiral_types.push(spiral_type);
        if self.recent_spiral_types.len() > 100 {
            self.recent_spiral_types.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spirachain_core::{Address, Amount};

    #[test]
    fn test_proof_of_spiral_creation() {
        let pos = ProofOfSpiral::new(spirachain_core::MIN_SPIRAL_COMPLEXITY, spirachain_core::MAX_SPIRAL_JUMP);
        assert_eq!(pos.min_complexity, spirachain_core::MIN_SPIRAL_COMPLEXITY);
    }

    #[test]
    fn test_semantic_clustering() {
        let pos = ProofOfSpiral::new(spirachain_core::MIN_SPIRAL_COMPLEXITY, spirachain_core::MAX_SPIRAL_JUMP);
        
        let mut transactions = Vec::new();
        for i in 0..10 {
            let from = Address::new([i; 32]);
            let to = Address::new([i + 1; 32]);
            let tx = Transaction::new(from, to, Amount::qbt(100), Amount::from_millis(1));
            transactions.push(tx);
        }

        let selected = pos.semantic_clustering(transactions).unwrap();
        assert_eq!(selected.len(), 10);
    }
}

