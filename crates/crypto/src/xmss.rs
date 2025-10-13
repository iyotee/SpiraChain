use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spirachain_core::{Result, SpiraChainError};

pub const XMSS_TREE_HEIGHT: usize = 20;
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

        let leaf = self.wots_verify(&signature.wots_signature, message);

        let computed_root =
            self.verify_auth_path(&leaf, &signature.auth_path, signature.index as usize);

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
            let mut hasher = Sha256::new();
            hasher.update(prf_seed);
            hasher.update(pub_seed);
            hasher.update(&(i as u64).to_be_bytes());
            let hash = hasher.finalize();

            let mut leaf = [0u8; 32];
            leaf.copy_from_slice(&hash);
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
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]);
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
        hasher.update(&self.secret_key.prf_seed);
        hasher.update(&index.to_be_bytes());
        hasher.finalize().to_vec()
    }

    fn wots_sign(&self, key: &[u8], message: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(message);
        let msg_hash = hasher.finalize();

        let mut signature = Vec::new();
        for i in 0..32 {
            let mut hasher = Sha256::new();
            hasher.update(key);
            hasher.update(&msg_hash[i..i + 1]);
            hasher.update(&(i as u32).to_be_bytes());
            signature.extend_from_slice(&hasher.finalize());
        }

        signature
    }

    fn wots_verify(&self, signature: &[u8], message: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(signature);
        hasher.update(message);
        let hash = hasher.finalize();

        let mut result = [0u8; 32];
        result.copy_from_slice(&hash);
        result
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
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]);
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
                hasher.update(&current_node);
                hasher.update(sibling);
            } else {
                hasher.update(sibling);
                hasher.update(&current_node);
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
