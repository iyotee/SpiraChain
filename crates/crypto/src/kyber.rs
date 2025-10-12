use spirachain_core::{Result, SpiraChainError};
use pqcrypto_kyber::kyber1024;
use pqcrypto_traits::kem::{PublicKey as PQPublicKey, SecretKey as PQSecretKey, SharedSecret as PQSharedSecret, Ciphertext as PQCiphertext};
use serde::{Deserialize, Serialize};

pub const KYBER_PUBLIC_KEY_SIZE: usize = kyber1024::public_key_bytes();
pub const KYBER_SECRET_KEY_SIZE: usize = kyber1024::secret_key_bytes();
pub const KYBER_CIPHERTEXT_SIZE: usize = kyber1024::ciphertext_bytes();
pub const KYBER_SHARED_SECRET_SIZE: usize = kyber1024::shared_secret_bytes();

#[derive(Clone)]
pub struct KyberKeyPair {
    public_key: kyber1024::PublicKey,
    secret_key: kyber1024::SecretKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KyberPublicKey {
    bytes: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct KyberSecretKey {
    bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KyberCiphertext {
    bytes: Vec<u8>,
}

#[derive(Clone)]
pub struct KyberSharedSecret {
    bytes: [u8; KYBER_SHARED_SECRET_SIZE],
}

impl KyberKeyPair {
    pub fn generate() -> Result<Self> {
        let (public_key, secret_key) = kyber1024::keypair();
        
        Ok(Self {
            public_key,
            secret_key,
        })
    }

    pub fn from_bytes(public_key_bytes: &[u8], secret_key_bytes: &[u8]) -> Result<Self> {
        if public_key_bytes.len() != KYBER_PUBLIC_KEY_SIZE {
            return Err(SpiraChainError::CryptoError(
                format!("Invalid Kyber public key size: {} (expected {})", 
                    public_key_bytes.len(), KYBER_PUBLIC_KEY_SIZE)
            ));
        }

        if secret_key_bytes.len() != KYBER_SECRET_KEY_SIZE {
            return Err(SpiraChainError::CryptoError(
                format!("Invalid Kyber secret key size: {} (expected {})",
                    secret_key_bytes.len(), KYBER_SECRET_KEY_SIZE)
            ));
        }

        let public_key = kyber1024::PublicKey::from_bytes(public_key_bytes)
            .map_err(|_| SpiraChainError::CryptoError("Failed to parse Kyber public key".to_string()))?;

        let secret_key = kyber1024::SecretKey::from_bytes(secret_key_bytes)
            .map_err(|_| SpiraChainError::CryptoError("Failed to parse Kyber secret key".to_string()))?;

        Ok(Self {
            public_key,
            secret_key,
        })
    }

    pub fn encapsulate(&self) -> Result<(KyberCiphertext, KyberSharedSecret)> {
        let (shared_secret, ciphertext) = kyber1024::encapsulate(&self.public_key);

        let ciphertext_bytes = ciphertext.as_bytes().to_vec();
        let mut shared_secret_bytes = [0u8; KYBER_SHARED_SECRET_SIZE];
        shared_secret_bytes.copy_from_slice(shared_secret.as_bytes());

        Ok((
            KyberCiphertext { bytes: ciphertext_bytes },
            KyberSharedSecret { bytes: shared_secret_bytes },
        ))
    }

    pub fn decapsulate(&self, ciphertext: &KyberCiphertext) -> Result<KyberSharedSecret> {
        if ciphertext.bytes.len() != KYBER_CIPHERTEXT_SIZE {
            return Err(SpiraChainError::CryptoError(
                format!("Invalid ciphertext size: {} (expected {})",
                    ciphertext.bytes.len(), KYBER_CIPHERTEXT_SIZE)
            ));
        }

        let ct = kyber1024::Ciphertext::from_bytes(&ciphertext.bytes)
            .map_err(|_| SpiraChainError::CryptoError("Failed to parse Kyber ciphertext".to_string()))?;

        let shared_secret = kyber1024::decapsulate(&ct, &self.secret_key);

        let mut shared_secret_bytes = [0u8; KYBER_SHARED_SECRET_SIZE];
        shared_secret_bytes.copy_from_slice(shared_secret.as_bytes());

        Ok(KyberSharedSecret { bytes: shared_secret_bytes })
    }

    pub fn public_key(&self) -> KyberPublicKey {
        KyberPublicKey {
            bytes: self.public_key.as_bytes().to_vec(),
        }
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.as_bytes().to_vec()
    }

    pub fn secret_key_bytes(&self) -> Vec<u8> {
        self.secret_key.as_bytes().to_vec()
    }
}

impl KyberPublicKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != KYBER_PUBLIC_KEY_SIZE {
            return Err(SpiraChainError::CryptoError(
                format!("Invalid Kyber public key size: {} (expected {})",
                    bytes.len(), KYBER_PUBLIC_KEY_SIZE)
            ));
        }

        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn encapsulate(&self) -> Result<(KyberCiphertext, KyberSharedSecret)> {
        let public_key = kyber1024::PublicKey::from_bytes(&self.bytes)
            .map_err(|_| SpiraChainError::CryptoError("Failed to parse public key".to_string()))?;

        let (shared_secret, ciphertext) = kyber1024::encapsulate(&public_key);

        let ciphertext_bytes = ciphertext.as_bytes().to_vec();
        let mut shared_secret_bytes = [0u8; KYBER_SHARED_SECRET_SIZE];
        shared_secret_bytes.copy_from_slice(shared_secret.as_bytes());

        Ok((
            KyberCiphertext { bytes: ciphertext_bytes },
            KyberSharedSecret { bytes: shared_secret_bytes },
        ))
    }
}

impl KyberCiphertext {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != KYBER_CIPHERTEXT_SIZE {
            return Err(SpiraChainError::CryptoError(
                format!("Invalid ciphertext size: {} (expected {})",
                    bytes.len(), KYBER_CIPHERTEXT_SIZE)
            ));
        }

        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

impl KyberSharedSecret {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn derive_key(&self, context: &[u8]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.bytes);
        hasher.update(context);
        let hash = hasher.finalize();
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash.as_bytes()[..32]);
        key
    }
}

impl std::fmt::Debug for KyberKeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KyberKeyPair")
            .field("public_key_size", &KYBER_PUBLIC_KEY_SIZE)
            .field("secret_key_size", &"[REDACTED]")
            .finish()
    }
}

impl std::fmt::Debug for KyberSecretKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KyberSecretKey")
            .field("size", &self.bytes.len())
            .field("bytes", &"[REDACTED]")
            .finish()
    }
}

impl std::fmt::Debug for KyberSharedSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KyberSharedSecret")
            .field("size", &KYBER_SHARED_SECRET_SIZE)
            .field("bytes", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber_keygen() {
        let keypair = KyberKeyPair::generate().unwrap();
        assert_eq!(keypair.public_key_bytes().len(), KYBER_PUBLIC_KEY_SIZE);
        assert_eq!(keypair.secret_key_bytes().len(), KYBER_SECRET_KEY_SIZE);
    }

    #[test]
    fn test_kyber_encapsulate_decapsulate() {
        let alice = KyberKeyPair::generate().unwrap();
        let bob = KyberKeyPair::generate().unwrap();

        let alice_public = alice.public_key();
        let (ciphertext, shared_secret_sender) = alice_public.encapsulate().unwrap();
        
        assert_eq!(ciphertext.as_bytes().len(), KYBER_CIPHERTEXT_SIZE);
        assert_eq!(shared_secret_sender.as_bytes().len(), KYBER_SHARED_SECRET_SIZE);

        let shared_secret_receiver = alice.decapsulate(&ciphertext).unwrap();
        
        assert_eq!(shared_secret_sender.as_bytes(), shared_secret_receiver.as_bytes());

        let shared_secret_bob = bob.decapsulate(&ciphertext);
        assert!(shared_secret_bob.is_ok());
        assert_ne!(shared_secret_sender.as_bytes(), shared_secret_bob.unwrap().as_bytes());
    }

    #[test]
    fn test_kyber_self_encapsulation() {
        let keypair = KyberKeyPair::generate().unwrap();

        let (ciphertext, shared_secret_sender) = keypair.encapsulate().unwrap();
        let shared_secret_receiver = keypair.decapsulate(&ciphertext).unwrap();

        assert_eq!(shared_secret_sender.as_bytes(), shared_secret_receiver.as_bytes());
    }

    #[test]
    fn test_kyber_public_key_encapsulation() {
        let keypair = KyberKeyPair::generate().unwrap();
        let public_key = keypair.public_key();

        let (ciphertext, shared_secret_sender) = public_key.encapsulate().unwrap();
        let shared_secret_receiver = keypair.decapsulate(&ciphertext).unwrap();

        assert_eq!(shared_secret_sender.as_bytes(), shared_secret_receiver.as_bytes());
    }

    #[test]
    fn test_kyber_key_derivation() {
        let keypair = KyberKeyPair::generate().unwrap();
        let (_, shared_secret) = keypair.encapsulate().unwrap();

        let key1 = shared_secret.derive_key(b"context1");
        let key2 = shared_secret.derive_key(b"context2");
        let key3 = shared_secret.derive_key(b"context1");

        assert_ne!(key1, key2);
        assert_eq!(key1, key3);
    }

    #[test]
    fn test_kyber_serialization() {
        let keypair = KyberKeyPair::generate().unwrap();
        
        let pub_bytes = keypair.public_key_bytes();
        let sec_bytes = keypair.secret_key_bytes();

        let keypair2 = KyberKeyPair::from_bytes(&pub_bytes, &sec_bytes).unwrap();

        let (ct1, ss1) = keypair.encapsulate().unwrap();
        let ss2 = keypair2.decapsulate(&ct1).unwrap();

        assert_eq!(ss1.as_bytes(), ss2.as_bytes());
    }
}

