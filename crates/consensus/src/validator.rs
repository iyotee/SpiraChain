use spirachain_core::{Address, Amount, Hash, Result, SpiraChainError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: Address,
    pub pubkey: Vec<u8>,
    pub stake: Amount,
    pub locked_until: u64,
    pub rewards_earned: Amount,
    pub slashing_events: Vec<SlashingEvent>,
    pub blocks_proposed: u64,
    pub expected_blocks: u64,
    pub reputation_score: f64,
    pub last_block_height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingEvent {
    pub reason: SlashingReason,
    pub amount: Amount,
    pub block_height: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlashingReason {
    InvalidSpiral,
    DoubleSigning,
    SemanticManipulation,
    Downtime,
    Censorship,
}

impl Validator {
    pub fn new(address: Address, pubkey: Vec<u8>, stake: Amount, current_block: u64) -> Result<Self> {
        if stake < Amount::new(spirachain_core::MIN_VALIDATOR_STAKE) {
            return Err(SpiraChainError::InsufficientStake(
                stake.value(),
                spirachain_core::MIN_VALIDATOR_STAKE
            ));
        }

        Ok(Self {
            address,
            pubkey,
            stake,
            locked_until: current_block + spirachain_core::LOCK_PERIOD_BLOCKS,
            rewards_earned: Amount::zero(),
            slashing_events: Vec::new(),
            blocks_proposed: 0,
            expected_blocks: 0,
            reputation_score: 1.0,
            last_block_height: current_block,
        })
    }

    pub fn can_unstake(&self, current_block: u64) -> bool {
        current_block >= self.locked_until && self.slashing_events.is_empty()
    }

    pub fn slash(&mut self, reason: SlashingReason, block_height: u64, timestamp: u64) -> Amount {
        let percentage = match reason {
            SlashingReason::InvalidSpiral => spirachain_core::SLASHING_INVALID_SPIRAL,
            SlashingReason::DoubleSigning => spirachain_core::SLASHING_DOUBLE_SIGNING,
            SlashingReason::SemanticManipulation => spirachain_core::SLASHING_SEMANTIC_MANIPULATION,
            SlashingReason::Downtime => spirachain_core::SLASHING_DOWNTIME,
            SlashingReason::Censorship => spirachain_core::SLASHING_CENSORSHIP,
        };

        let slash_amount = Amount::new((self.stake.value() as f64 * percentage) as u128);
        
        if let Some(new_stake) = self.stake.checked_sub(slash_amount) {
            self.stake = new_stake;
        } else {
            self.stake = Amount::zero();
        }

        self.slashing_events.push(SlashingEvent {
            reason,
            amount: slash_amount,
            block_height,
            timestamp,
        });

        self.reputation_score = (self.reputation_score * 0.5).max(0.0);

        slash_amount
    }

    pub fn add_reward(&mut self, reward: Amount) {
        if let Some(new_rewards) = self.rewards_earned.checked_add(reward) {
            self.rewards_earned = new_rewards;
        }
    }

    pub fn update_reputation(&mut self, spiral_quality: f64, semantic_coherence: f64, timeliness: f64) {
        let uptime = if self.expected_blocks > 0 {
            self.blocks_proposed as f64 / self.expected_blocks as f64
        } else {
            1.0
        };

        let new_score = 
            0.3 * spiral_quality +
            0.3 * semantic_coherence +
            0.2 * timeliness +
            0.2 * uptime;

        self.reputation_score = 0.9 * self.reputation_score + 0.1 * new_score;
    }

    pub fn increment_blocks_proposed(&mut self, block_height: u64) {
        self.blocks_proposed += 1;
        self.last_block_height = block_height;
    }

    pub fn increment_expected_blocks(&mut self) {
        self.expected_blocks += 1;
    }

    pub fn is_active(&self) -> bool {
        self.stake >= Amount::new(spirachain_core::MIN_VALIDATOR_STAKE) &&
        self.reputation_score > 0.3 &&
        self.slashing_events.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSet {
    pub validators: Vec<Validator>,
    pub total_stake: Amount,
}

impl ValidatorSet {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            total_stake: Amount::zero(),
        }
    }

    pub fn add_validator(&mut self, validator: Validator) -> Result<()> {
        if self.validators.len() >= spirachain_core::MAX_VALIDATORS {
            return Err(SpiraChainError::ConsensusError(
                "Maximum validator count reached".to_string()
            ));
        }

        if let Some(total) = self.total_stake.checked_add(validator.stake) {
            self.total_stake = total;
        }
        
        self.validators.push(validator);
        Ok(())
    }

    pub fn remove_validator(&mut self, address: &Address) -> Result<()> {
        if let Some(pos) = self.validators.iter().position(|v| v.address == *address) {
            let validator = self.validators.remove(pos);
            if let Some(total) = self.total_stake.checked_sub(validator.stake) {
                self.total_stake = total;
            }
            Ok(())
        } else {
            Err(SpiraChainError::ValidatorNotFound(address.to_string()))
        }
    }

    pub fn get_validator(&self, address: &Address) -> Option<&Validator> {
        self.validators.iter().find(|v| v.address == *address)
    }

    pub fn get_validator_mut(&mut self, address: &Address) -> Option<&mut Validator> {
        self.validators.iter_mut().find(|v| v.address == *address)
    }

    pub fn active_validators(&self) -> Vec<&Validator> {
        self.validators.iter().filter(|v| v.is_active()).collect()
    }

    pub fn len(&self) -> usize {
        self.validators.len()
    }

    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }
}

impl Default for ValidatorSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let address = Address::new([1u8; 32]);
        let pubkey = vec![0u8; 32];
        let stake = Amount::qbt(10_000);
        
        let validator = Validator::new(address, pubkey, stake, 0).unwrap();
        assert_eq!(validator.stake, stake);
        assert!(validator.is_active());
    }

    #[test]
    fn test_validator_slashing() {
        let address = Address::new([1u8; 32]);
        let pubkey = vec![0u8; 32];
        let stake = Amount::qbt(10_000);
        
        let mut validator = Validator::new(address, pubkey, stake, 0).unwrap();
        let original_stake = validator.stake;
        
        validator.slash(SlashingReason::InvalidSpiral, 100, 1000);
        
        assert!(validator.stake < original_stake);
        assert_eq!(validator.slashing_events.len(), 1);
    }

    #[test]
    fn test_validator_set() {
        let mut set = ValidatorSet::new();
        
        let address1 = Address::new([1u8; 32]);
        let validator1 = Validator::new(address1, vec![0u8; 32], Amount::qbt(10_000), 0).unwrap();
        
        let address2 = Address::new([2u8; 32]);
        let validator2 = Validator::new(address2, vec![1u8; 32], Amount::qbt(20_000), 0).unwrap();
        
        set.add_validator(validator1).unwrap();
        set.add_validator(validator2).unwrap();
        
        assert_eq!(set.len(), 2);
        assert_eq!(set.total_stake, Amount::qbt(30_000));
    }
}

