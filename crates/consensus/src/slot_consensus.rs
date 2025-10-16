// Slot-based Consensus (Cardano-style)
// Each validator gets a turn to produce blocks in a round-robin fashion

use spirachain_core::Address;
use std::time::{SystemTime, UNIX_EPOCH};

/// Slot duration in seconds (testnet: 30s, mainnet: 60s)
pub const SLOT_DURATION_TESTNET: u64 = 30;
pub const SLOT_DURATION_MAINNET: u64 = 60;

/// Slot-based consensus manager
#[derive(Debug, Clone)]
pub struct SlotConsensus {
    /// Network type (testnet or mainnet)
    #[allow(dead_code)]
    network: String,
    /// List of active validators (sorted by address for determinism)
    validators: Vec<Address>,
    /// Slot duration in seconds
    slot_duration: u64,
}

impl SlotConsensus {
    /// Create a new slot consensus manager
    pub fn new(network: &str) -> Self {
        let slot_duration = if network == "mainnet" {
            SLOT_DURATION_MAINNET
        } else {
            SLOT_DURATION_TESTNET
        };

        Self {
            network: network.to_string(),
            validators: Vec::new(),
            slot_duration,
        }
    }

    /// Add a validator to the active set
    pub fn add_validator(&mut self, address: Address) {
        if !self.validators.contains(&address) {
            self.validators.push(address);
            // Sort for determinism (everyone must have the same order)
            self.validators.sort_by_key(|a| *a.as_bytes());
        }
    }

    /// Remove a validator from the active set
    pub fn remove_validator(&mut self, address: &Address) {
        self.validators.retain(|v| v != address);
    }

    /// Get the current slot number based on timestamp
    pub fn get_current_slot(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Slot 0 started at Unix epoch
        now / self.slot_duration
    }

    /// Get the validator that should produce the block for a given slot
    pub fn get_slot_leader(&self, slot: u64) -> Option<Address> {
        if self.validators.is_empty() {
            return None;
        }

        // Round-robin: slot 0 → validator 0, slot 1 → validator 1, etc.
        let index = (slot as usize) % self.validators.len();
        Some(self.validators[index])
    }

    /// Check if the given validator is the leader for the current slot
    pub fn is_slot_leader(&self, validator: &Address) -> bool {
        let current_slot = self.get_current_slot();

        match self.get_slot_leader(current_slot) {
            Some(leader) => leader == *validator,
            None => {
                // If no validators registered, anyone can produce (bootstrap)
                true
            }
        }
    }

    /// Get the current slot leader
    pub fn get_current_leader(&self) -> Option<Address> {
        let current_slot = self.get_current_slot();
        self.get_slot_leader(current_slot)
    }

    /// Get time until next slot in seconds
    pub fn time_until_next_slot(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let current_slot = self.get_current_slot();
        let next_slot_start = (current_slot + 1) * self.slot_duration;

        next_slot_start.saturating_sub(now)
    }

    /// Get the list of all validators
    pub fn get_validators(&self) -> &[Address] {
        &self.validators
    }

    /// Get the number of active validators
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_calculation() {
        let consensus = SlotConsensus::new("testnet");
        let slot = consensus.get_current_slot();

        // Should return a reasonable slot number
        assert!(slot > 0);
    }

    #[test]
    fn test_round_robin() {
        let mut consensus = SlotConsensus::new("testnet");

        let addr1 = Address([1u8; 32]);
        let addr2 = Address([2u8; 32]);
        let addr3 = Address([3u8; 32]);

        consensus.add_validator(addr1);
        consensus.add_validator(addr2);
        consensus.add_validator(addr3);

        // Slot 0 should be validator 0
        assert_eq!(consensus.get_slot_leader(0), Some(addr1));
        // Slot 1 should be validator 1
        assert_eq!(consensus.get_slot_leader(1), Some(addr2));
        // Slot 2 should be validator 2
        assert_eq!(consensus.get_slot_leader(2), Some(addr3));
        // Slot 3 should wrap around to validator 0
        assert_eq!(consensus.get_slot_leader(3), Some(addr1));
    }

    #[test]
    fn test_deterministic_ordering() {
        let mut consensus1 = SlotConsensus::new("testnet");
        let mut consensus2 = SlotConsensus::new("testnet");

        let addr1 = Address([1u8; 32]);
        let addr2 = Address([2u8; 32]);

        // Add in different order
        consensus1.add_validator(addr1);
        consensus1.add_validator(addr2);

        consensus2.add_validator(addr2);
        consensus2.add_validator(addr1);

        // Should produce the same leader for the same slot
        assert_eq!(consensus1.get_slot_leader(0), consensus2.get_slot_leader(0));
        assert_eq!(consensus1.get_slot_leader(1), consensus2.get_slot_leader(1));
    }
}
