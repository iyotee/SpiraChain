use crate::{Hash, PiCoordinate, Result, SpiraChainError, SpiralMetadata, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u64,
    pub previous_block_hash: Hash,
    pub merkle_root: Hash,
    pub spiral_root: Hash,
    pub state_root: Hash, // Merkle root of complete WorldState (all account balances)
    pub timestamp: u64,
    pub pi_coordinates: PiCoordinate,
    pub spiral: SpiralMetadata,
    pub validator_pubkey: Vec<u8>,
    pub signature: Vec<u8>,
    pub nonce: u64,
    pub difficulty_target: u32,
    pub tx_count: u32,
    pub block_height: u64,
}

impl BlockHeader {
    pub fn new(previous_block_hash: Hash, block_height: u64) -> Self {
        Self {
            version: 1,
            previous_block_hash,
            merkle_root: Hash::zero(),
            spiral_root: Hash::zero(),
            state_root: Hash::zero(), // Will be calculated after applying transactions
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            pi_coordinates: PiCoordinate::zero(),
            spiral: SpiralMetadata::new(crate::SpiralType::Archimedean),
            validator_pubkey: Vec::new(),
            signature: Vec::new(),
            nonce: 0,
            difficulty_target: u32::MAX / 1000, // Testnet: facile à miner
            tx_count: 0,
            block_height,
        }
    }

    pub fn hash(&self) -> Hash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.version.to_be_bytes());
        hasher.update(self.previous_block_hash.as_bytes());
        hasher.update(self.merkle_root.as_bytes());
        hasher.update(self.spiral_root.as_bytes());
        hasher.update(self.state_root.as_bytes()); // Include state root in block hash
        hasher.update(&self.timestamp.to_be_bytes());
        hasher.update(&self.pi_coordinates.x.to_be_bytes());
        hasher.update(&self.pi_coordinates.y.to_be_bytes());
        hasher.update(&self.pi_coordinates.z.to_be_bytes());
        hasher.update(&self.pi_coordinates.t.to_be_bytes());
        hasher.update(&self.nonce.to_be_bytes());
        hasher.update(&self.difficulty_target.to_be_bytes());
        hasher.update(&self.block_height.to_be_bytes());
        hasher.finalize().into()
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data).map_err(|e| SpiraChainError::SerializationError(e.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(previous_block_hash: Hash, block_height: u64) -> Self {
        Self {
            header: BlockHeader::new(previous_block_hash, block_height),
            transactions: Vec::new(),
        }
    }

    pub fn with_transactions(mut self, transactions: Vec<Transaction>) -> Self {
        self.header.tx_count = transactions.len() as u32;
        self.transactions = transactions;
        self
    }

    pub fn with_spiral(mut self, spiral: SpiralMetadata) -> Self {
        self.header.spiral = spiral;
        self
    }

    pub fn with_pi_coordinates(mut self, coords: PiCoordinate) -> Self {
        self.header.pi_coordinates = coords;
        self
    }

    pub fn with_validator(mut self, pubkey: Vec<u8>) -> Self {
        self.header.validator_pubkey = pubkey;
        self
    }

    pub fn compute_merkle_root(&mut self) {
        if self.transactions.is_empty() {
            self.header.merkle_root = Hash::zero();
            return;
        }

        let mut hashes: Vec<Hash> = self.transactions.iter().map(|tx| tx.tx_hash).collect();

        while hashes.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in hashes.chunks(2) {
                let mut hasher = blake3::Hasher::new();
                hasher.update(chunk[0].as_bytes());
                if chunk.len() > 1 {
                    hasher.update(chunk[1].as_bytes());
                } else {
                    hasher.update(chunk[0].as_bytes());
                }
                next_level.push(hasher.finalize().into());
            }

            hashes = next_level;
        }

        self.header.merkle_root = hashes[0];
    }

    pub fn compute_spiral_root(&mut self) {
        let data = bincode::serialize(&self.header.spiral).unwrap_or_default();
        self.header.spiral_root = blake3::hash(&data).into();
    }

    pub fn hash(&self) -> Hash {
        self.header.hash()
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data).map_err(|e| SpiraChainError::SerializationError(e.to_string()))
    }

    pub fn validate(&self) -> Result<()> {
        if self.header.version == 0 {
            return Err(SpiraChainError::InvalidBlock("Invalid version".to_string()));
        }

        if self.header.previous_block_hash == Hash::zero() && self.header.block_height != 0 {
            return Err(SpiraChainError::InvalidBlock(
                "Invalid previous block hash".to_string(),
            ));
        }

        if self.transactions.len() > crate::MAX_TX_PER_BLOCK {
            return Err(SpiraChainError::InvalidBlock(format!(
                "Too many transactions: {} > {}",
                self.transactions.len(),
                crate::MAX_TX_PER_BLOCK
            )));
        }

        if self.header.spiral.complexity < crate::MIN_SPIRAL_COMPLEXITY {
            return Err(SpiraChainError::SpiralComplexityTooLow(
                self.header.spiral.complexity,
                crate::MIN_SPIRAL_COMPLEXITY,
            ));
        }

        if self.header.signature.is_empty() {
            return Err(SpiraChainError::InvalidSignature);
        }

        for tx in &self.transactions {
            tx.validate()?;
        }

        let mut block_clone = self.clone();
        block_clone.compute_merkle_root();
        if block_clone.header.merkle_root != self.header.merkle_root {
            return Err(SpiraChainError::InvalidBlock(
                "Invalid merkle root".to_string(),
            ));
        }

        Ok(())
    }

    pub fn avg_semantic_coherence(&self) -> f64 {
        if self.transactions.is_empty() {
            return 0.0;
        }

        let sum: f64 = self
            .transactions
            .iter()
            .map(|tx| tx.semantic_coherence())
            .sum();

        sum / (self.transactions.len() as f64)
    }

    pub fn size(&self) -> usize {
        self.serialize().len()
    }

    pub fn is_genesis(&self) -> bool {
        self.header.block_height == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Address, Amount};

    #[test]
    fn test_block_creation() {
        let prev_hash = Hash::new([0u8; 32]);
        let block = Block::new(prev_hash, 1);

        assert_eq!(block.header.block_height, 1);
        assert_eq!(block.header.previous_block_hash, prev_hash);
    }

    #[test]
    fn test_merkle_root_computation() {
        let prev_hash = Hash::new([0u8; 32]);
        let mut block = Block::new(prev_hash, 1);

        let from = Address::new([1u8; 32]);
        let to = Address::new([2u8; 32]);
        let amount = Amount::qbt(100);
        let fee = Amount::from_millis(1);

        let mut tx1 = Transaction::new(from, to, amount, fee);
        tx1.compute_hash();
        let mut tx2 = Transaction::new(to, from, amount, fee);
        tx2.compute_hash();

        block = block.with_transactions(vec![tx1, tx2]);
        block.compute_merkle_root();

        assert_ne!(block.header.merkle_root, Hash::zero());
    }

    #[test]
    fn test_genesis_block() {
        let prev_hash = Hash::zero();
        let block = Block::new(prev_hash, 0);

        assert!(block.is_genesis());
    }
}
