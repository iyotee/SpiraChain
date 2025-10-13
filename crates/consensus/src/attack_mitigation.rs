use spirachain_core::{Address, Block, Hash, Result, SpiraChainError, Transaction};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{error, info, warn};

pub const DOUBLE_SPEND_WINDOW: Duration = Duration::from_secs(300);
pub const CHECKPOINT_INTERVAL: u64 = 100;
pub const SLASHING_AMOUNT_PERCENT: f64 = 30.0;

pub struct AttackMitigationSystem {
    checkpoints: HashMap<u64, Hash>,
    double_spend_detector: DoubleSpendDetector,
    validator_monitor: ValidatorMonitor,
    last_checkpoint_height: u64,
}

struct DoubleSpendDetector {
    recent_transactions: HashMap<Hash, TransactionInfo>,
    addresses_monitored: HashMap<Address, Vec<Hash>>,
}

#[derive(Debug, Clone)]
struct TransactionInfo {
    transaction: Transaction,
    block_height: Option<u64>,
    first_seen: Instant,
    times_seen: usize,
}

impl TransactionInfo {
    /// Check if this transaction is suspicious (seen multiple times)
    fn is_suspicious(&self) -> bool {
        self.times_seen > 1
    }

    /// Get the age of this transaction info
    fn age(&self) -> Duration {
        self.first_seen.elapsed()
    }

    /// Check if transaction is from a high-risk address
    fn is_high_risk(&self) -> bool {
        // Check if amount is suspiciously high
        self.transaction.amount.value() > 1_000_000_000_000 // > 1M QBT
    }
}

struct ValidatorMonitor {
    suspicious_validators: HashMap<Address, SuspicionRecord>,
    blocks_per_validator: HashMap<Address, Vec<u64>>,
}

#[derive(Debug, Clone)]
struct SuspicionRecord {
    #[allow(dead_code)]
    validator: Address,
    offense_count: usize,
    last_offense: Instant,
    total_slashed: u128,
}

impl SuspicionRecord {
    fn new(validator: Address) -> Self {
        Self {
            validator,
            offense_count: 0,
            last_offense: Instant::now(),
            total_slashed: 0,
        }
    }

    /// Record a new offense
    fn record_offense(&mut self, slashed_amount: u128) {
        self.offense_count += 1;
        self.last_offense = Instant::now();
        self.total_slashed += slashed_amount;
    }

    /// Check if validator should be permanently banned
    fn should_ban(&self) -> bool {
        self.offense_count >= 3 || self.total_slashed > 10_000_000_000_000 // > 10M QBT slashed
    }

    /// Get time since last offense
    #[allow(dead_code)]
    fn time_since_last_offense(&self) -> Duration {
        self.last_offense.elapsed()
    }

    /// Get validator address
    #[allow(dead_code)]
    pub fn validator(&self) -> &Address {
        &self.validator
    }
}

impl AttackMitigationSystem {
    pub fn new() -> Self {
        info!("🛡️  Attack Mitigation System initialized");
        info!("   Double-spend window: {}s", DOUBLE_SPEND_WINDOW.as_secs());
        info!("   Checkpoint interval: {} blocks", CHECKPOINT_INTERVAL);
        info!("   Slashing rate: {}%", SLASHING_AMOUNT_PERCENT);

        Self {
            checkpoints: HashMap::new(),
            double_spend_detector: DoubleSpendDetector {
                recent_transactions: HashMap::new(),
                addresses_monitored: HashMap::new(),
            },
            validator_monitor: ValidatorMonitor {
                suspicious_validators: HashMap::new(),
                blocks_per_validator: HashMap::new(),
            },
            last_checkpoint_height: 0,
        }
    }

    pub fn process_block(&mut self, block: &Block) -> Result<()> {
        self.check_for_double_spends(block)?;

        self.monitor_validator_behavior(block);

        if block
            .header
            .block_height
            .is_multiple_of(CHECKPOINT_INTERVAL)
        {
            self.create_checkpoint(block);
        }

        self.cleanup_old_data();

        Ok(())
    }

    fn check_for_double_spends(&mut self, block: &Block) -> Result<()> {
        for tx in &block.transactions {
            let tx_hash = tx.hash();

            if let Some(existing) = self
                .double_spend_detector
                .recent_transactions
                .get_mut(&tx_hash)
            {
                existing.times_seen += 1;

                if existing.is_suspicious() {
                    error!("🚨 DOUBLE-SPEND DETECTED!");
                    error!("   Transaction: {}", tx_hash);
                    error!("   From: {}", tx.from);
                    error!("   Times seen: {}", existing.times_seen);
                    error!("   Block height: {:?}", existing.block_height);
                    error!("   Age: {:?}", existing.age());

                    // Check if it's also high-risk
                    if existing.is_high_risk() {
                        error!("   ⚠️  HIGH-RISK TRANSACTION (amount > 1M QBT)");
                    }

                    return Err(SpiraChainError::ConsensusError(format!(
                        "Double-spend detected: {}",
                        tx_hash
                    )));
                }
            } else {
                let tx_info = TransactionInfo {
                    transaction: tx.clone(),
                    block_height: Some(block.header.block_height),
                    first_seen: Instant::now(),
                    times_seen: 1,
                };
                self.double_spend_detector
                    .recent_transactions
                    .insert(tx_hash, tx_info);
            }

            self.double_spend_detector
                .addresses_monitored
                .entry(tx.from)
                .or_default()
                .push(tx_hash);
        }

        Ok(())
    }

    fn monitor_validator_behavior(&mut self, block: &Block) {
        let validator_pubkey_hash = blake3::hash(&block.header.validator_pubkey);
        let validator_addr = Address::new(*validator_pubkey_hash.as_bytes());

        self.validator_monitor
            .blocks_per_validator
            .entry(validator_addr)
            .or_default()
            .push(block.header.block_height);

        let blocks_produced = self
            .validator_monitor
            .blocks_per_validator
            .get(&validator_addr)
            .map(|v| v.len())
            .unwrap_or(0);

        let total_validators = self.validator_monitor.blocks_per_validator.len();

        if total_validators > 1 {
            let expected_share = 1.0 / total_validators as f64;
            let actual_share = blocks_produced as f64 / block.header.block_height as f64;

            if actual_share > expected_share * 2.0 {
                warn!(
                    "⚠️  Validator {} producing {:.1}% of blocks (suspicious)",
                    validator_addr,
                    actual_share * 100.0
                );

                // Record the offense
                let record = self
                    .validator_monitor
                    .suspicious_validators
                    .entry(validator_addr)
                    .or_insert_with(|| SuspicionRecord::new(validator_addr));

                // Calculate slashing amount based on offense severity
                let slashing_amount = (actual_share * 1_000_000_000_000.0) as u128; // Proportional to dominance
                record.record_offense(slashing_amount);

                warn!(
                    "   Offense count: {} | Total slashed: {} QBT",
                    record.offense_count,
                    record.total_slashed / 1_000_000_000
                );

                // Check if validator should be banned
                if record.should_ban() {
                    error!("🚨 VALIDATOR {} SHOULD BE BANNED!", validator_addr);
                    error!("   Total offenses: {}", record.offense_count);
                    error!(
                        "   Total slashed: {} QBT",
                        record.total_slashed / 1_000_000_000
                    );
                }
            }
        }
    }

    pub fn create_checkpoint(&mut self, block: &Block) {
        let block_hash = block.hash();
        self.checkpoints
            .insert(block.header.block_height, block_hash);
        self.last_checkpoint_height = block.header.block_height;

        info!(
            "📍 Checkpoint created at height {}",
            block.header.block_height
        );
        info!("   Hash: {}", block_hash);
        info!("   Finality: Irreversible after this point");
    }

    pub fn is_finalized(&self, block_height: u64) -> bool {
        block_height <= self.last_checkpoint_height
    }

    pub fn get_checkpoint(&self, height: u64) -> Option<Hash> {
        let checkpoint_height = (height / CHECKPOINT_INTERVAL) * CHECKPOINT_INTERVAL;
        self.checkpoints.get(&checkpoint_height).copied()
    }

    pub fn slash_validator(&mut self, validator: Address, reason: &str) -> u128 {
        let slashing_amount = (SLASHING_AMOUNT_PERCENT / 100.0 * 10_000.0 * 1e18) as u128;

        error!("⚔️  SLASHING VALIDATOR!");
        error!("   Validator: {}", validator);
        error!("   Reason: {}", reason);
        error!("   Amount: {} QBT", slashing_amount as f64 / 1e18);

        self.validator_monitor
            .suspicious_validators
            .entry(validator)
            .or_insert_with(|| SuspicionRecord {
                validator,
                offense_count: 0,
                last_offense: Instant::now(),
                total_slashed: 0,
            })
            .total_slashed += slashing_amount;

        slashing_amount
    }

    pub fn detect_51_attack(&self) -> Option<Address> {
        let total_blocks: usize = self
            .validator_monitor
            .blocks_per_validator
            .values()
            .map(|v| v.len())
            .sum();

        for (validator, blocks) in &self.validator_monitor.blocks_per_validator {
            let validator_share = blocks.len() as f64 / total_blocks as f64;

            if validator_share > 0.51 {
                error!("🚨 51% ATTACK DETECTED!");
                error!("   Attacker: {}", validator);
                error!("   Control: {:.1}%", validator_share * 100.0);
                return Some(*validator);
            }
        }

        None
    }

    fn cleanup_old_data(&mut self) {
        let now = Instant::now();

        self.double_spend_detector
            .recent_transactions
            .retain(|_, info| now.duration_since(info.first_seen) < DOUBLE_SPEND_WINDOW);

        self.double_spend_detector
            .addresses_monitored
            .retain(|_, tx_hashes| {
                tx_hashes.retain(|hash| {
                    self.double_spend_detector
                        .recent_transactions
                        .contains_key(hash)
                });
                !tx_hashes.is_empty()
            });
    }

    pub fn get_suspicious_validators(&self) -> Vec<Address> {
        self.validator_monitor
            .suspicious_validators
            .keys()
            .copied()
            .collect()
    }

    pub fn checkpoint_count(&self) -> usize {
        self.checkpoints.len()
    }
}

impl Default for AttackMitigationSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spirachain_core::Amount;

    fn create_test_block(height: u64, validator_pubkey: Vec<u8>) -> Block {
        let mut block = Block::new(Hash::zero(), height);
        block.header.validator_pubkey = validator_pubkey;
        block
    }

    #[test]
    fn test_attack_mitigation_init() {
        let mitigation = AttackMitigationSystem::new();
        assert_eq!(mitigation.checkpoint_count(), 0);
    }

    #[test]
    fn test_checkpoint_creation() {
        let mut mitigation = AttackMitigationSystem::new();
        let block = create_test_block(100, vec![1, 2, 3]);

        mitigation.create_checkpoint(&block);

        assert_eq!(mitigation.checkpoint_count(), 1);
        assert!(mitigation.is_finalized(100));
        assert!(!mitigation.is_finalized(101));
    }

    #[test]
    fn test_double_spend_detection() {
        let mut mitigation = AttackMitigationSystem::new();

        let tx = Transaction::new(
            Address::new([1u8; 32]),
            Address::new([2u8; 32]),
            Amount::new(100),
            Amount::new(1),
        );

        let mut block1 = create_test_block(1, vec![1, 2, 3]);
        block1.transactions.push(tx.clone());

        let result1 = mitigation.process_block(&block1);
        assert!(result1.is_ok());

        let mut block2 = create_test_block(2, vec![4, 5, 6]);
        block2.transactions.push(tx.clone());

        let result2 = mitigation.process_block(&block2);
        assert!(result2.is_err());
    }

    #[test]
    fn test_51_attack_detection() {
        let mut mitigation = AttackMitigationSystem::new();

        let validator1_key = vec![1u8; 32];
        let validator2_key = vec![2u8; 32];

        for i in 0..60 {
            let block = create_test_block(i, validator1_key.clone());
            let _ = mitigation.process_block(&block);
        }

        for i in 60..100 {
            let block = create_test_block(i, validator2_key.clone());
            let _ = mitigation.process_block(&block);
        }

        let attacker = mitigation.detect_51_attack();
        assert!(attacker.is_some());
    }

    #[test]
    fn test_validator_slashing() {
        let mut mitigation = AttackMitigationSystem::new();
        let validator = Address::new([42u8; 32]);

        let slashed = mitigation.slash_validator(validator, "Double-spend attempt");
        assert!(slashed > 0);

        let suspicious = mitigation.get_suspicious_validators();
        assert_eq!(suspicious.len(), 1);
        assert_eq!(suspicious[0], validator);
    }
}
