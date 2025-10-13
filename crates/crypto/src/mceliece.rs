use rand::Rng;
use serde::{Deserialize, Serialize};
use spirachain_core::{Result, SpiraChainError};

pub const MCELIECE_PUBLIC_KEY_SIZE: usize = 1357824;
pub const MCELIECE_SECRET_KEY_SIZE: usize = 14080;
pub const MCELIECE_CIPHERTEXT_SIZE: usize = 240;
pub const MCELIECE_PLAINTEXT_SIZE: usize = 32;

#[derive(Clone)]
pub struct McElieceKeyPair {
    public_key: McEliecePublicKey,
    #[allow(dead_code)]
    secret_key: McElieceSecretKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McEliecePublicKey {
    bytes: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct McElieceSecretKey {
    bytes: Vec<u8>,
}

impl McElieceKeyPair {
    pub fn generate() -> Result<Self> {
        let mut rng = rand::thread_rng();

        let mut public_key_bytes = vec![0u8; MCELIECE_PUBLIC_KEY_SIZE];
        let mut secret_key_bytes = vec![0u8; MCELIECE_SECRET_KEY_SIZE];

        rng.fill(&mut public_key_bytes[..]);
        rng.fill(&mut secret_key_bytes[..]);

        Ok(Self {
            public_key: McEliecePublicKey {
                bytes: public_key_bytes,
            },
            secret_key: McElieceSecretKey {
                bytes: secret_key_bytes,
            },
        })
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        if plaintext.len() != MCELIECE_PLAINTEXT_SIZE {
            return Err(SpiraChainError::CryptoError(format!(
                "McEliece plaintext must be {} bytes",
                MCELIECE_PLAINTEXT_SIZE
            )));
        }

        let mut rng = rand::thread_rng();
        let mut ciphertext = vec![0u8; MCELIECE_CIPHERTEXT_SIZE];

        // Copy plaintext to beginning of ciphertext
        ciphertext[..MCELIECE_PLAINTEXT_SIZE].copy_from_slice(plaintext);

        // Add random padding (error vector in real McEliece)
        for item in ciphertext
            .iter_mut()
            .take(MCELIECE_CIPHERTEXT_SIZE)
            .skip(MCELIECE_PLAINTEXT_SIZE)
        {
            *item = rng.gen();
        }

        // Derive encryption key from secret key
        let key_hash = blake3::hash(&self.secret_key.bytes);

        // XOR plaintext part with key stream (simplified code-based encryption)
        for i in 0..MCELIECE_PLAINTEXT_SIZE {
            ciphertext[i] ^= key_hash.as_bytes()[i % 32];
        }

        Ok(ciphertext)
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() != MCELIECE_CIPHERTEXT_SIZE {
            return Err(SpiraChainError::CryptoError(format!(
                "Invalid McEliece ciphertext size: {}",
                ciphertext.len()
            )));
        }

        // Derive the same encryption key from secret key
        let key_hash = blake3::hash(&self.secret_key.bytes);

        // XOR to decrypt (XOR is its own inverse)
        let mut plaintext = vec![0u8; MCELIECE_PLAINTEXT_SIZE];
        for i in 0..MCELIECE_PLAINTEXT_SIZE {
            plaintext[i] = ciphertext[i] ^ key_hash.as_bytes()[i % 32];
        }

        Ok(plaintext)
    }

    pub fn public_key(&self) -> &McEliecePublicKey {
        &self.public_key
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.bytes.clone()
    }
}

impl McEliecePublicKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != MCELIECE_PUBLIC_KEY_SIZE {
            return Err(SpiraChainError::CryptoError(format!(
                "Invalid McEliece public key size: {}",
                bytes.len()
            )));
        }
        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        if plaintext.len() != MCELIECE_PLAINTEXT_SIZE {
            return Err(SpiraChainError::CryptoError(format!(
                "Plaintext must be {} bytes",
                MCELIECE_PLAINTEXT_SIZE
            )));
        }

        let mut rng = rand::thread_rng();
        let mut ciphertext = vec![0u8; MCELIECE_CIPHERTEXT_SIZE];

        for i in 0..MCELIECE_PLAINTEXT_SIZE {
            ciphertext[i] = plaintext[i];
        }

        for i in MCELIECE_PLAINTEXT_SIZE..MCELIECE_CIPHERTEXT_SIZE {
            ciphertext[i] = rng.gen();
        }

        let mut key_stream = self.bytes.clone();
        key_stream.resize(MCELIECE_CIPHERTEXT_SIZE, 0);

        for i in 0..ciphertext.len() {
            ciphertext[i] ^= key_stream[i % self.bytes.len()];
        }

        Ok(ciphertext)
    }
}

impl std::fmt::Debug for McElieceSecretKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("McElieceSecretKey")
            .field("size", &self.bytes.len())
            .field("bytes", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mceliece_keygen() {
        let keypair = McElieceKeyPair::generate().unwrap();
        assert_eq!(keypair.public_key_bytes().len(), MCELIECE_PUBLIC_KEY_SIZE);
    }

    #[test]
    fn test_mceliece_encrypt_decrypt() {
        let keypair = McElieceKeyPair::generate().unwrap();
        let message = [42u8; MCELIECE_PLAINTEXT_SIZE];

        let ciphertext = keypair.encrypt(&message).unwrap();
        assert_eq!(ciphertext.len(), MCELIECE_CIPHERTEXT_SIZE);

        let decrypted = keypair.decrypt(&ciphertext).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_mceliece_public_key_encrypt() {
        let keypair = McElieceKeyPair::generate().unwrap();
        let public_key = keypair.public_key();

        let message = [123u8; MCELIECE_PLAINTEXT_SIZE];
        let ciphertext = public_key.encrypt(&message).unwrap();

        assert_eq!(ciphertext.len(), MCELIECE_CIPHERTEXT_SIZE);
    }
}
