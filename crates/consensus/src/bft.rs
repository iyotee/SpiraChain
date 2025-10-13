use spirachain_core::{Block, Hash, Result, SpiraChainError, Address};
use spirachain_crypto::KeyPair;
use crate::ValidatorSet;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

pub const BFT_QUORUM_THRESHOLD: f64 = 0.67;
pub const BFT_TIMEOUT_SECONDS: u64 = 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BFTMessage {
    PrePrepare { view: u64, sequence: u64, block: Block, signature: Vec<u8> },
    Prepare { view: u64, sequence: u64, block_hash: Hash, validator: Address, signature: Vec<u8> },
    Commit { view: u64, sequence: u64, block_hash: Hash, validator: Address, signature: Vec<u8> },
    ViewChange { new_view: u64, validator: Address, proof: Vec<u8> },
}

#[derive(Debug, Clone)]
pub struct Vote {
    pub validator: Address,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

pub struct BFTConsensus {
    view_number: u64,
    sequence_number: u64,
    validators: ValidatorSet,
    pre_prepare_received: HashMap<Hash, (Block, Vote)>,
    prepare_votes: HashMap<Hash, Vec<Vote>>,
    commit_votes: HashMap<Hash, Vec<Vote>>,
    committed_blocks: HashMap<Hash, Block>,
    local_validator: Address,
}

impl BFTConsensus {
    pub fn new(validators: ValidatorSet, local_validator: Address) -> Self {
        info!("🛡️  Initializing BFT Consensus");
        info!("   Validators: {}", validators.validators.len());
        info!("   Quorum threshold: {:.0}%", BFT_QUORUM_THRESHOLD * 100.0);

        Self {
            view_number: 0,
            sequence_number: 0,
            validators,
            pre_prepare_received: HashMap::new(),
            prepare_votes: HashMap::new(),
            commit_votes: HashMap::new(),
            committed_blocks: HashMap::new(),
            local_validator,
        }
    }

    pub fn propose_block(&mut self, block: Block, keypair: &KeyPair) -> Result<BFTMessage> {
        info!("📢 Proposing block for BFT consensus");
        info!("   View: {}, Sequence: {}", self.view_number, self.sequence_number);
        info!("   Block height: {}", block.header.block_height);

        let block_hash = block.hash();
        let signature = keypair.sign(block_hash.as_bytes());

        let message = BFTMessage::PrePrepare {
            view: self.view_number,
            sequence: self.sequence_number,
            block,
            signature,
        };

        self.sequence_number += 1;

        Ok(message)
    }

    pub fn handle_pre_prepare(&mut self, view: u64, sequence: u64, block: Block, signature: Vec<u8>) -> Result<Option<BFTMessage>> {
        if view != self.view_number {
            warn!("⚠️  Received PrePrepare for wrong view: {} (expected {})", view, self.view_number);
            return Ok(None);
        }

        let block_hash = block.hash();

        let vote = Vote {
            validator: self.local_validator,
            signature: signature.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.pre_prepare_received.insert(block_hash, (block.clone(), vote));

        info!("✅ PrePrepare received for block {}", block.header.block_height);

        let prepare_msg = BFTMessage::Prepare {
            view,
            sequence,
            block_hash,
            validator: self.local_validator,
            signature,
        };

        Ok(Some(prepare_msg))
    }

    pub fn handle_prepare(&mut self, view: u64, sequence: u64, block_hash: Hash, validator: Address, signature: Vec<u8>) -> Result<Option<BFTMessage>> {
        if view != self.view_number {
            return Ok(None);
        }

        let vote = Vote {
            validator,
            signature: signature.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.prepare_votes.entry(block_hash).or_insert_with(Vec::new).push(vote);

        let vote_count = self.prepare_votes.get(&block_hash).map(|v| v.len()).unwrap_or(0);
        let quorum = self.calculate_quorum();

        if vote_count >= quorum {
            info!("✅ Prepare quorum reached for block: {}/{}", vote_count, quorum);

            let commit_msg = BFTMessage::Commit {
                view,
                sequence,
                block_hash,
                validator: self.local_validator,
                signature,
            };

            return Ok(Some(commit_msg));
        }

        Ok(None)
    }

    pub fn handle_commit(&mut self, view: u64, _sequence: u64, block_hash: Hash, validator: Address, signature: Vec<u8>) -> Result<bool> {
        if view != self.view_number {
            return Ok(false);
        }

        let vote = Vote {
            validator,
            signature,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.commit_votes.entry(block_hash).or_insert_with(Vec::new).push(vote);

        let vote_count = self.commit_votes.get(&block_hash).map(|v| v.len()).unwrap_or(0);
        let quorum = self.calculate_quorum();

        if vote_count >= quorum {
            info!("🎉 Commit quorum reached! Block is FINAL: {}/{}", vote_count, quorum);

            if let Some((block, _)) = self.pre_prepare_received.get(&block_hash) {
                self.committed_blocks.insert(block_hash, block.clone());
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn is_committed(&self, block_hash: &Hash) -> bool {
        self.committed_blocks.contains_key(block_hash)
    }

    pub fn get_committed_block(&self, block_hash: &Hash) -> Option<Block> {
        self.committed_blocks.get(block_hash).cloned()
    }

    fn calculate_quorum(&self) -> usize {
        let total_validators = self.validators.validators.len();
        ((total_validators as f64 * BFT_QUORUM_THRESHOLD).ceil() as usize).max(1)
    }

    pub fn handle_view_change(&mut self, new_view: u64) -> Result<()> {
        if new_view <= self.view_number {
            return Err(SpiraChainError::ConsensusError(
                format!("Invalid view change: {} -> {}", self.view_number, new_view)
            ));
        }

        warn!("🔄 View change: {} → {}", self.view_number, new_view);

        self.view_number = new_view;
        self.pre_prepare_received.clear();
        self.prepare_votes.clear();
        self.commit_votes.clear();

        Ok(())
    }

    pub fn current_view(&self) -> u64 {
        self.view_number
    }

    pub fn sequence_number(&self) -> u64 {
        self.sequence_number
    }

    pub fn validator_count(&self) -> usize {
        self.validators.validators.len()
    }

    pub fn quorum_size(&self) -> usize {
        self.calculate_quorum()
    }

    pub fn get_prepare_votes(&self, block_hash: &Hash) -> usize {
        self.prepare_votes.get(block_hash).map(|v| v.len()).unwrap_or(0)
    }

    pub fn get_commit_votes(&self, block_hash: &Hash) -> usize {
        self.commit_votes.get(block_hash).map(|v| v.len()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spirachain_core::Amount;

    fn create_test_validator_set() -> ValidatorSet {
        let mut validators = ValidatorSet::new();
        
        for i in 0..4 {
            let keypair = KeyPair::generate();
            let validator = Validator {
                address: keypair.to_address(),
                pubkey: keypair.public_key().as_bytes().to_vec(),
                stake: Amount::new(10_000 * 10u128.pow(18)),
                locked_until: 0,
                rewards_earned: Amount::new(0),
                slashing_events: Vec::new(),
                blocks_proposed: 0,
                expected_blocks: 0,
                reputation_score: 1.0,
                last_block_height: 0,
            };
            let _ = validators.add_validator(validator);
        }

        validators
    }

    #[test]
    fn test_bft_initialization() {
        let validators = create_test_validator_set();
        let local_validator = validators.validators[0].address;
        let bft = BFTConsensus::new(validators, local_validator);

        assert_eq!(bft.current_view(), 0);
        assert_eq!(bft.validator_count(), 4);
        assert_eq!(bft.quorum_size(), 3);
    }

    #[test]
    fn test_bft_quorum_calculation() {
        let validators = create_test_validator_set();
        let local_validator = validators.validators[0].address;
        let bft = BFTConsensus::new(validators, local_validator);

        let quorum = bft.quorum_size();
        assert_eq!(quorum, 3);
    }

    #[test]
    fn test_bft_propose_block() {
        let validators = create_test_validator_set();
        let local_validator = validators.validators[0].address;
        let mut bft = BFTConsensus::new(validators, local_validator);

        let keypair = KeyPair::generate();
        let block = Block::new(Hash::zero(), 1);

        let message = bft.propose_block(block, &keypair).unwrap();

        match message {
            BFTMessage::PrePrepare { view, sequence, .. } => {
                assert_eq!(view, 0);
                assert_eq!(sequence, 0);
            }
            _ => panic!("Expected PrePrepare message"),
        }
    }

    #[test]
    fn test_bft_view_change() {
        let validators = create_test_validator_set();
        let local_validator = validators.validators[0].address;
        let mut bft = BFTConsensus::new(validators, local_validator);

        assert_eq!(bft.current_view(), 0);

        bft.handle_view_change(1).unwrap();
        assert_eq!(bft.current_view(), 1);

        let result = bft.handle_view_change(1);
        assert!(result.is_err());
    }
}

