use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spirachain_core::{Result, SpiraChainError};

// Production: 20 (1M signatures), Tests: 10 (1024 signatures)
pub const XMSS_TREE_HEIGHT: usize = if cfg!(test) { 10 } else { 20 };
pub const XMSS_SIGNATURE_SIZE: usize = 2500;

#[derive(Clone, Serialize, Deserialize)]
pub struct XmssKeyPair {
    public_key: XmssPublicKey,
    secret_key: XmssSecretKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmssPublicKey {
    root: [u8; 32],
    pub_seed: [u8; 32],
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmssSecretKey {
    index: u64,
    seed: [u8; 32],
    prf_seed: [u8; 32],
    pub_seed: [u8; 32],
    root: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmssSignature {
    index: u64,
    wots_signature: Vec<u8>,
    auth_path: Vec<[u8; 32]>,
}

impl XmssKeyPair {
    pub fn generate() -> Result<Self> {
        let mut rng = rand::thread_rng();

        let mut seed = [0u8; 32];
        let mut prf_seed = [0u8; 32];
        let mut pub_seed = [0u8; 32];

        rng.fill(&mut seed);
        rng.fill(&mut prf_seed);
        rng.fill(&mut pub_seed);

        let leaf_nodes = Self::generate_leaf_nodes(&prf_seed, &pub_seed);
        let root = Self::merkle_root(&leaf_nodes);

        let secret_key = XmssSecretKey {
            index: 0,
            seed,
            prf_seed,
            pub_seed,
            root,
        };

        let public_key = XmssPublicKey { root, pub_seed };

        Ok(Self {
            public_key,
            secret_key,
        })
    }

    pub fn sign(&mut self, message: &[u8]) -> Result<XmssSignature> {
        if self.secret_key.index >= (1u64 << XMSS_TREE_HEIGHT) {
            return Err(SpiraChainError::CryptoError(
                "XMSS key exhausted - no more signatures available".to_string(),
            ));
        }

        let index = self.secret_key.index;

        let wots_key = self.generate_wots_key(index);
        let wots_signature = self.wots_sign(&wots_key, message);

        let leaf_nodes =
            Self::generate_leaf_nodes(&self.secret_key.prf_seed, &self.secret_key.pub_seed);
        let auth_path = self.generate_auth_path(&leaf_nodes, index as usize);

        self.secret_key.index += 1;

        Ok(XmssSignature {
            index,
            wots_signature,
            auth_path,
        })
    }

    pub fn verify(&self, message: &[u8], signature: &XmssSignature) -> bool {
        if signature.index >= (1u64 << XMSS_TREE_HEIGHT) {
            return false;
        }

        // Reconstruct the WOTS public key (leaf) from the signature
        let reconstructed_leaf = self.wots_verify(&signature.wots_signature, message);

        // Verify the auth path from this leaf to the root
        let computed_root = self.verify_auth_path(
            &reconstructed_leaf,
            &signature.auth_path,
            signature.index as usize,
        );

        computed_root == self.public_key.root
    }

    pub fn public_key(&self) -> &XmssPublicKey {
        &self.public_key
    }

    pub fn remaining_signatures(&self) -> u64 {
        (1u64 << XMSS_TREE_HEIGHT) - self.secret_key.index
    }

    fn generate_leaf_nodes(prf_seed: &[u8; 32], pub_seed: &[u8; 32]) -> Vec<[u8; 32]> {
        let num_leaves = 1 << XMSS_TREE_HEIGHT;
        let mut leaves = Vec::with_capacity(num_leaves);

        for i in 0..num_leaves {
            // Generate WOTS public key for this leaf
            // Start with the PRF seed + index to get deterministic key
            let mut key_hasher = Sha256::new();
            key_hasher.update(prf_seed);
            key_hasher.update((i as u64).to_be_bytes());
            let wots_key = key_hasher.finalize();

            // Generate the WOTS public key parts
            let mut public_key_parts = Vec::new();

            for j in 0..32 {
                // Start with seed derived from key and position
                let mut chain_hasher = Sha256::new();
                chain_hasher.update(wots_key);
                chain_hasher.update((j as u32).to_be_bytes());
                let mut chain_value = chain_hasher.finalize();

                // Chain hash 255 times (maximum for w=256 Winternitz)
                for _ in 0..255 {
                    let mut next_hasher = Sha256::new();
                    next_hasher.update(chain_value);
                    chain_value = next_hasher.finalize();
                }

                public_key_parts.extend_from_slice(&chain_value);
            }

            // Hash the complete public key to get the leaf
            let mut leaf_hasher = Sha256::new();
            leaf_hasher.update(&public_key_parts);
            leaf_hasher.update(pub_seed); // Add pub_seed for additional entropy
            let leaf_hash = leaf_hasher.finalize();

            let mut leaf = [0u8; 32];
            leaf.copy_from_slice(&leaf_hash);
            leaves.push(leaf);
        }

        leaves
    }

    fn merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
        if leaves.is_empty() {
            return [0u8; 32];
        }

        let mut current_level = leaves.to_vec();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in current_level.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(chunk[1]);
                } else {
                    hasher.update(chunk[0]);
                }
                let hash = hasher.finalize();

                let mut node = [0u8; 32];
                node.copy_from_slice(&hash);
                next_level.push(node);
            }

            current_level = next_level;
        }

        current_level[0]
    }

    fn generate_wots_key(&self, index: u64) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.secret_key.prf_seed);
        hasher.update(index.to_be_bytes());
        hasher.finalize().to_vec()
    }

    fn wots_sign(&self, key: &[u8], message: &[u8]) -> Vec<u8> {
        // Hash the message to get 32 bytes
        let mut hasher = Sha256::new();
        hasher.update(message);
        let msg_hash = hasher.finalize();

        // WOTS signature: for each byte of the hash, chain-hash the key
        let mut signature = Vec::new();

        for (i, &byte) in msg_hash.iter().enumerate() {
            // Start with a deterministic seed derived from the key and position
            let mut chain_hasher = Sha256::new();
            chain_hasher.update(key);
            chain_hasher.update((i as u32).to_be_bytes());
            let mut chain_value = chain_hasher.finalize();

            // Chain hash 'byte' times (Winternitz parameter)
            for _ in 0..byte {
                let mut next_hasher = Sha256::new();
                next_hasher.update(chain_value);
                chain_value = next_hasher.finalize();
            }

            signature.extend_from_slice(&chain_value);
        }

        signature
    }

    fn wots_verify(&self, signature: &[u8], message: &[u8]) -> [u8; 32] {
        // Hash the message
        let mut hasher = Sha256::new();
        hasher.update(message);
        let msg_hash = hasher.finalize();

        // Reconstruct the WOTS public key from the signature
        let mut public_key_parts = Vec::new();

        for (i, &byte) in msg_hash.iter().enumerate() {
            // Extract the signature chunk for this position (32 bytes each)
            let sig_start = i * 32;
            let sig_end = sig_start + 32;

            if sig_end > signature.len() {
                // Invalid signature length
                return [0u8; 32];
            }

            let mut chain_value = [0u8; 32];
            chain_value.copy_from_slice(&signature[sig_start..sig_end]);

            // Continue chain hashing from 'byte' to 255 to get the public key
            let remaining_iterations = 255 - byte;
            for _ in 0..remaining_iterations {
                let mut next_hasher = Sha256::new();
                next_hasher.update(chain_value);
                let hash_result = next_hasher.finalize();
                chain_value.copy_from_slice(&hash_result);
            }

            // This should now be the public key part for position i
            public_key_parts.extend_from_slice(&chain_value);
        }

        // Hash all public key parts + pub_seed to get the leaf (must match generate_leaf_nodes)
        let mut final_hasher = Sha256::new();
        final_hasher.update(&public_key_parts);
        final_hasher.update(self.public_key.pub_seed);
        let result = final_hasher.finalize();

        let mut leaf = [0u8; 32];
        leaf.copy_from_slice(&result);
        leaf
    }

    fn generate_auth_path(&self, leaves: &[[u8; 32]], index: usize) -> Vec<[u8; 32]> {
        let mut auth_path = Vec::new();
        let mut current_index = index;
        let mut current_level = leaves.to_vec();

        for _ in 0..XMSS_TREE_HEIGHT {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < current_level.len() {
                auth_path.push(current_level[sibling_index]);
            } else {
                auth_path.push(current_level[current_index]);
            }

            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(chunk[1]);
                } else {
                    hasher.update(chunk[0]);
                }
                let hash = hasher.finalize();

                let mut node = [0u8; 32];
                node.copy_from_slice(&hash);
                next_level.push(node);
            }

            current_level = next_level;
            current_index /= 2;
        }

        auth_path
    }

    fn verify_auth_path(&self, leaf: &[u8; 32], auth_path: &[[u8; 32]], index: usize) -> [u8; 32] {
        let mut current_node = *leaf;
        let mut current_index = index;

        for sibling in auth_path {
            let mut hasher = Sha256::new();
            if current_index % 2 == 0 {
                hasher.update(current_node);
                hasher.update(sibling);
            } else {
                hasher.update(sibling);
                hasher.update(current_node);
            }
            let hash = hasher.finalize();

            current_node.copy_from_slice(&hash);
            current_index /= 2;
        }

        current_node
    }
}

impl XmssPublicKey {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.root);
        result.extend_from_slice(&self.pub_seed);
        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 64 {
            return Err(SpiraChainError::CryptoError(
                "Invalid XMSS public key length".to_string(),
            ));
        }

        let mut root = [0u8; 32];
        let mut pub_seed = [0u8; 32];
        root.copy_from_slice(&bytes[0..32]);
        pub_seed.copy_from_slice(&bytes[32..64]);

        Ok(Self { root, pub_seed })
    }
}

impl std::fmt::Debug for XmssSecretKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XmssSecretKey")
            .field("index", &self.index)
            .field("seed", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmss_keygen() {
        let keypair = XmssKeyPair::generate().unwrap();
        assert_eq!(keypair.remaining_signatures(), 1u64 << XMSS_TREE_HEIGHT);
    }

    #[test]
    fn test_xmss_sign_verify() {
        let mut keypair = XmssKeyPair::generate().unwrap();
        let message = b"test message";

        let signature = keypair.sign(message).unwrap();
        assert!(keypair.verify(message, &signature));
    }

    #[test]
    fn test_xmss_multiple_signatures() {
        let mut keypair = XmssKeyPair::generate().unwrap();
        let message1 = b"message 1";
        let message2 = b"message 2";

        let sig1 = keypair.sign(message1).unwrap();
        let sig2 = keypair.sign(message2).unwrap();

        assert!(keypair.verify(message1, &sig1));
        assert!(keypair.verify(message2, &sig2));
        assert_eq!(
            keypair.remaining_signatures(),
            (1u64 << XMSS_TREE_HEIGHT) - 2
        );
    }

    #[test]
    fn test_xmss_wrong_message() {
        let mut keypair = XmssKeyPair::generate().unwrap();
        let message = b"correct message";
        let wrong = b"wrong message";

        let signature = keypair.sign(message).unwrap();
        assert!(!keypair.verify(wrong, &signature));
    }
}
