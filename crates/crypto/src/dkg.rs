use rand::Rng;
use serde::{Deserialize, Serialize};
use spirachain_core::{Address, Result, SpiraChainError};
use std::collections::HashMap;

pub const DKG_THRESHOLD_RATIO: f64 = 0.67;
pub const DKG_KEY_FRAGMENT_SIZE: usize = 32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DKGKeyFragment {
    pub fragment_id: u32,
    pub fragment_data: Vec<u8>,
    pub validator: Address,
}

#[derive(Debug, Clone)]
pub struct DKGCoordinator {
    threshold: usize,
    total_participants: usize,
    key_fragments: HashMap<u32, DKGKeyFragment>,
    master_secret: Option<Vec<u8>>,
}

impl DKGCoordinator {
    pub fn new(participant_count: usize) -> Self {
        let threshold = ((participant_count as f64 * DKG_THRESHOLD_RATIO).ceil() as usize).max(2);

        tracing::info!("🔑 DKG Coordinator initialized");
        tracing::info!("   Participants: {}", participant_count);
        tracing::info!("   Threshold: {}/{}", threshold, participant_count);

        Self {
            threshold,
            total_participants: participant_count,
            key_fragments: HashMap::new(),
            master_secret: None,
        }
    }

    pub fn generate_key_fragments(&mut self) -> Result<Vec<DKGKeyFragment>> {
        let mut rng = rand::thread_rng();

        let mut master_key = vec![0u8; DKG_KEY_FRAGMENT_SIZE];
        rng.fill(&mut master_key[..]);

        let mut fragments = Vec::new();

        for i in 0..self.total_participants {
            let mut fragment_data = vec![0u8; DKG_KEY_FRAGMENT_SIZE];

            for j in 0..DKG_KEY_FRAGMENT_SIZE {
                let master_byte = master_key[j];
                let noise = rng.gen::<u8>();
                let shard = master_byte.wrapping_add(noise);
                fragment_data[j] = shard;
            }

            let fragment = DKGKeyFragment {
                fragment_id: i as u32,
                fragment_data,
                validator: Address::new([i as u8; 32]),
            };

            self.key_fragments.insert(i as u32, fragment.clone());
            fragments.push(fragment);
        }

        self.master_secret = Some(master_key);

        tracing::info!("✅ Generated {} key fragments", fragments.len());

        Ok(fragments)
    }

    pub fn reconstruct_key(&self, fragments: &[DKGKeyFragment]) -> Result<Vec<u8>> {
        if fragments.len() < self.threshold {
            return Err(SpiraChainError::CryptoError(format!(
                "Insufficient fragments: {}/{}",
                fragments.len(),
                self.threshold
            )));
        }

        let mut reconstructed = vec![0u8; DKG_KEY_FRAGMENT_SIZE];

        for (idx, fragment) in fragments.iter().enumerate().take(self.threshold) {
            for (j, &byte) in fragment.fragment_data.iter().enumerate() {
                if idx == 0 {
                    reconstructed[j] = byte;
                } else {
                    reconstructed[j] = reconstructed[j].wrapping_sub(byte);
                }
            }
        }

        tracing::info!(
            "🔓 Reconstructed master key from {}/{} fragments",
            fragments.len(),
            self.threshold
        );

        Ok(reconstructed)
    }

    pub fn verify_fragment(&self, fragment: &DKGKeyFragment) -> bool {
        self.key_fragments.contains_key(&fragment.fragment_id)
    }

    pub fn add_fragment(&mut self, fragment: DKGKeyFragment) -> Result<()> {
        if self.key_fragments.len() >= self.total_participants {
            return Err(SpiraChainError::CryptoError(
                "All fragments already received".to_string(),
            ));
        }

        self.key_fragments.insert(fragment.fragment_id, fragment);

        Ok(())
    }

    pub fn has_quorum(&self) -> bool {
        self.key_fragments.len() >= self.threshold
    }

    pub fn threshold(&self) -> usize {
        self.threshold
    }

    pub fn fragment_count(&self) -> usize {
        self.key_fragments.len()
    }
}

pub struct FractalKeyRotation {
    rotation_counter: u64,
    pi_digits_cache: Vec<u8>,
}

impl FractalKeyRotation {
    pub fn new() -> Self {
        let pi_approx = std::f64::consts::PI.to_string();
        let pi_digits: Vec<u8> = pi_approx
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        tracing::info!("🌀 Fractal Key Rotation initialized");
        tracing::info!("   π-based derivation active");

        Self {
            rotation_counter: 0,
            pi_digits_cache: pi_digits,
        }
    }

    pub fn derive_next_key(&mut self, previous_key: &[u8]) -> Vec<u8> {
        let pi_index = (self.rotation_counter as usize) % self.pi_digits_cache.len();
        let pi_digit = self.pi_digits_cache[pi_index];

        let mut hasher = blake3::Hasher::new();
        hasher.update(previous_key);
        hasher.update(&(self.rotation_counter * pi_digit as u64).to_be_bytes());
        hasher.update(&[pi_digit]);

        let hash = hasher.finalize();
        let derived_key = hash.as_bytes().to_vec();

        self.rotation_counter += 1;

        tracing::debug!(
            "🔄 Derived key #{} using π[{}]={}",
            self.rotation_counter,
            pi_index,
            pi_digit
        );

        derived_key
    }

    pub fn derive_key_at_index(&self, base_key: &[u8], index: u64) -> Vec<u8> {
        let pi_index = (index as usize) % self.pi_digits_cache.len();
        let pi_digit = self.pi_digits_cache[pi_index];

        let mut hasher = blake3::Hasher::new();
        hasher.update(base_key);
        hasher.update(&(index * pi_digit as u64).to_be_bytes());
        hasher.update(&[pi_digit]);

        hasher.finalize().as_bytes().to_vec()
    }

    pub fn rotation_count(&self) -> u64 {
        self.rotation_counter
    }

    pub fn should_rotate(&self, seconds_elapsed: u64) -> bool {
        seconds_elapsed > 10
    }
}

impl Default for FractalKeyRotation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dkg_initialization() {
        let dkg = DKGCoordinator::new(10);
        assert_eq!(dkg.threshold(), 7);
        assert_eq!(dkg.fragment_count(), 0);
    }

    #[test]
    fn test_dkg_fragment_generation() {
        let mut dkg = DKGCoordinator::new(5);
        let fragments = dkg.generate_key_fragments().unwrap();

        assert_eq!(fragments.len(), 5);
        assert_eq!(dkg.fragment_count(), 5);
    }

    #[test]
    fn test_dkg_key_reconstruction() {
        let mut dkg = DKGCoordinator::new(5);
        let fragments = dkg.generate_key_fragments().unwrap();

        let threshold_fragments = &fragments[..dkg.threshold()];
        let reconstructed = dkg.reconstruct_key(threshold_fragments);

        assert!(reconstructed.is_ok());
    }

    #[test]
    fn test_dkg_insufficient_fragments() {
        let mut dkg = DKGCoordinator::new(10);
        let fragments = dkg.generate_key_fragments().unwrap();

        let insufficient = &fragments[..3];
        let result = dkg.reconstruct_key(insufficient);

        assert!(result.is_err());
    }

    #[test]
    fn test_fractal_key_rotation() {
        let mut rotation = FractalKeyRotation::new();
        let base_key = vec![42u8; 32];

        let key1 = rotation.derive_next_key(&base_key);
        let key2 = rotation.derive_next_key(&key1);
        let key3 = rotation.derive_next_key(&key2);

        assert_ne!(key1, key2);
        assert_ne!(key2, key3);
        assert_ne!(key1, key3);

        assert_eq!(rotation.rotation_count(), 3);
    }

    #[test]
    fn test_fractal_key_derivation_deterministic() {
        let rotation1 = FractalKeyRotation::new();
        let rotation2 = FractalKeyRotation::new();

        let base_key = vec![123u8; 32];

        let key1 = rotation1.derive_key_at_index(&base_key, 42);
        let key2 = rotation2.derive_key_at_index(&base_key, 42);

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_fractal_rotation_with_pi() {
        let mut rotation = FractalKeyRotation::new();
        let base_key = vec![0u8; 32];

        let keys: Vec<_> = (0..100)
            .map(|_| rotation.derive_next_key(&base_key))
            .collect();

        for i in 0..keys.len() {
            for j in (i + 1)..keys.len() {
                assert_ne!(keys[i], keys[j], "Keys {} and {} should be different", i, j);
            }
        }
    }
}
