use spirachain_core::{Amount, Block};

pub struct RewardCalculator;

impl RewardCalculator {
    pub fn calculate_block_reward(
        block: &Block,
        recent_spiral_types: &[spirachain_core::SpiralType],
    ) -> Amount {
        let base_reward = Self::base_reward_at_height(block.header.block_height);

        let complexity_multiplier = (block.header.spiral.complexity / 100.0).min(1.5);
        let coherence_multiplier = block.avg_semantic_coherence();

        let novelty_bonus = if !recent_spiral_types.contains(&block.header.spiral.spiral_type) {
            1.2
        } else {
            1.0
        };

        let full_block_bonus = if block.transactions.len() > 80 {
            1.1
        } else {
            1.0
        };

        let total_multiplier =
            complexity_multiplier * coherence_multiplier * novelty_bonus * full_block_bonus;

        let final_multiplier = total_multiplier.min(2.0);

        let reward_value = (base_reward.value() as f64 * final_multiplier) as u128;
        Amount::new(reward_value)
    }

    fn base_reward_at_height(height: u64) -> Amount {
        let halvings = height / spirachain_core::HALVING_BLOCKS;
        let base = spirachain_core::INITIAL_BLOCK_REWARD;

        let reward = if halvings < 64 { base >> halvings } else { 0 };

        Amount::new(reward)
    }

    pub fn calculate_tx_fee(
        tx_size: usize,
        purpose_length: usize,
        semantic_coherence: f64,
    ) -> Amount {
        let gas_per_byte = 100u128;
        let semantic_gas_per_char = 50u128;

        let base_fee = (tx_size as u128) * gas_per_byte;
        let semantic_fee = (purpose_length as u128) * semantic_gas_per_char;

        let discount = if semantic_coherence > 0.9 {
            0.9
        } else if semantic_coherence > 0.8 {
            0.95
        } else {
            1.0
        };

        let total_fee = ((base_fee + semantic_fee) as f64 * discount) as u128;

        Amount::new(total_fee.max(spirachain_core::MIN_TX_FEE))
    }

    pub fn distribute_fees(total_fees: Amount) -> (Amount, Amount, Amount) {
        let validator_share = Amount::new((total_fees.value() as f64 * 0.5) as u128);
        let burn_share = Amount::new((total_fees.value() as f64 * 0.3) as u128);
        let treasury_share = Amount::new((total_fees.value() as f64 * 0.2) as u128);

        (validator_share, burn_share, treasury_share)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spirachain_core::Hash;

    #[test]
    fn test_block_reward_calculation() {
        let mut block = Block::new(Hash::zero(), 0);
        block.header.spiral.complexity = 75.0;
        block.header.spiral.semantic_coherence = 1.0; // Ensure coherence is set
        
        // Add a transaction to ensure semantic coherence is not 0
        let from = spirachain_core::Address::new([1u8; 32]);
        let to = spirachain_core::Address::new([2u8; 32]);
        let amount = spirachain_core::Amount::qbt(100);
        let fee = spirachain_core::Amount::from_millis(1);
        
        let mut tx = spirachain_core::Transaction::new(from, to, amount, fee);
        tx.semantic_vector = vec![0.5; 100]; // Set semantic vector for coherence
        tx.compute_hash();
        
        block = block.with_transactions(vec![tx]);

        let recent_types = vec![];
        let reward = RewardCalculator::calculate_block_reward(&block, &recent_types);

        // Debug output to see what's happening
        println!("Base reward: {}", RewardCalculator::base_reward_at_height(0).value());
        println!("Block coherence: {}", block.avg_semantic_coherence());
        println!("Calculated reward: {}", reward.value());
        
        assert!(reward > Amount::zero(), "Reward should be positive, got: {}", reward.value());
    }

    #[test]
    fn test_halving() {
        let reward_0 = RewardCalculator::base_reward_at_height(0);
        let reward_halving =
            RewardCalculator::base_reward_at_height(spirachain_core::HALVING_BLOCKS);

        assert_eq!(reward_halving.value(), reward_0.value() / 2);
    }

    #[test]
    fn test_fee_calculation() {
        let fee = RewardCalculator::calculate_tx_fee(1000, 100, 0.9);
        assert!(fee >= Amount::new(spirachain_core::MIN_TX_FEE));
    }

    #[test]
    fn test_fee_distribution() {
        let total = Amount::qbt(100);
        let (validator, burn, treasury) = RewardCalculator::distribute_fees(total);

        let sum = validator.value() + burn.value() + treasury.value();
        assert!(sum <= total.value());
    }
}
