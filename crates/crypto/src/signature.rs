// Signature types for different cryptographic schemes
use crate::{KeyPair, PublicKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureScheme {
    Ed25519,
    Xmss,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub scheme: SignatureScheme,
    pub data: Vec<u8>,
}

impl Signature {
    pub fn new_ed25519(data: Vec<u8>) -> Self {
        Self {
            scheme: SignatureScheme::Ed25519,
            data,
        }
    }

    pub fn new_xmss(data: Vec<u8>) -> Self {
        Self {
            scheme: SignatureScheme::Xmss,
            data,
        }
    }

    pub fn verify(&self, public_key: &[u8], message: &[u8]) -> bool {
        match self.scheme {
            SignatureScheme::Ed25519 => {
                if public_key.len() != 32 {
                    return false;
                }
                let mut pk_bytes = [0u8; 32];
                pk_bytes.copy_from_slice(public_key);
                let pk = PublicKey(pk_bytes);
                PublicKey::verify(&pk, message, &self.data)
            }
            SignatureScheme::Xmss => {
                false
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.clone()
    }
}

pub fn sign_ed25519(keypair: &KeyPair, message: &[u8]) -> Signature {
    let sig_data = keypair.sign(message);
    Signature::new_ed25519(sig_data)
}

pub fn verify_ed25519(public_key: &PublicKey, message: &[u8], signature: &[u8]) -> bool {
    PublicKey::verify(public_key, message, signature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_signature() {
        let keypair = KeyPair::generate();
        let message = b"test message";
        
        let signature = sign_ed25519(&keypair, message);
        assert!(signature.verify(keypair.public_key().as_bytes(), message));
    }

    #[test]
    fn test_signature_wrong_message() {
        let keypair = KeyPair::generate();
        let message = b"test message";
        let wrong = b"wrong message";
        
        let signature = sign_ed25519(&keypair, message);
        assert!(!signature.verify(keypair.public_key().as_bytes(), wrong));
    }
}

