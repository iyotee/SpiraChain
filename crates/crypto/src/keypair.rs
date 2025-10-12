use spirachain_core::{Address, Result};
use serde::{Deserialize, Serialize};
use ed25519_dalek::{Signer, Verifier};
use rand::rngs::OsRng;

#[derive(Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub public_key: PublicKey,
    secret_key: SecretKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey(pub [u8; 32]);

#[derive(Clone, Serialize, Deserialize)]
pub struct SecretKey([u8; 32]);

impl KeyPair {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        
        let secret = signing_key.to_bytes();
        let public = signing_key.verifying_key().to_bytes();
        
        Self {
            public_key: PublicKey(public),
            secret_key: SecretKey(secret),
        }
    }

    pub fn from_secret(secret: [u8; 32]) -> Result<Self> {
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&secret);
        let public = signing_key.verifying_key().to_bytes();
        
        Ok(Self {
            public_key: PublicKey(public),
            secret_key: SecretKey(secret),
        })
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&self.secret_key.0);
        let signature = signing_key.sign(message);
        signature.to_bytes().to_vec()
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        PublicKey::verify(&self.public_key, message, signature)
    }

    pub fn to_address(&self) -> Address {
        let hash = blake3::hash(&self.public_key.0);
        Address::new(*hash.as_bytes())
    }
}

impl PublicKey {
    pub fn to_address(&self) -> Address {
        let hash = blake3::hash(&self.0);
        Address::new(*hash.as_bytes())
    }

    pub fn verify(public_key: &PublicKey, message: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 64 {
            return false;
        }

        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(signature);

        let verifying_key = match ed25519_dalek::VerifyingKey::from_bytes(&public_key.0) {
            Ok(key) => key,
            Err(_) => return false,
        };

        let sig = ed25519_dalek::Signature::from_bytes(&sig_bytes);

        verifying_key.verify(message, &sig).is_ok()
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl SecretKey {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Debug for KeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyPair")
            .field("public_key", &self.public_key)
            .field("secret_key", &"[REDACTED]")
            .finish()
    }
}

impl std::fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = KeyPair::generate();
        assert_ne!(keypair.public_key.0, [0u8; 32]);
    }

    #[test]
    fn test_sign_and_verify() {
        let keypair = KeyPair::generate();
        let message = b"test message";
        
        let signature = keypair.sign(message);
        assert!(keypair.verify(message, &signature));
    }

    #[test]
    fn test_verify_wrong_message() {
        let keypair = KeyPair::generate();
        let message = b"test message";
        let wrong_message = b"wrong message";
        
        let signature = keypair.sign(message);
        assert!(!keypair.verify(wrong_message, &signature));
    }

    #[test]
    fn test_to_address() {
        let keypair = KeyPair::generate();
        let address = keypair.to_address();
        
        assert_ne!(address, Address::zero());
    }
}

