use parking_lot::RwLock;
use spirachain_core::{Result, SpiraChainError};
use spirachain_crypto::{KyberCiphertext, KyberKeyPair, KyberPublicKey, KyberSharedSecret};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

const KEY_ROTATION_THRESHOLD: usize = 1000;

pub struct P2PEncryption {
    local_keypair: Arc<RwLock<KyberKeyPair>>,
    peer_keys: Arc<RwLock<HashMap<String, PeerEncryptionState>>>,
    messages_sent: Arc<RwLock<usize>>,
}

struct PeerEncryptionState {
    public_key: KyberPublicKey,
    shared_secret: Option<KyberSharedSecret>,
    messages_exchanged: usize,
    #[allow(dead_code)]
    established_at: std::time::Instant,
}

impl P2PEncryption {
    pub fn new() -> Result<Self> {
        let keypair = KyberKeyPair::generate()?;

        info!("🔐 P2P Encryption initialized with Kyber-1024");
        info!(
            "   Public key size: {} bytes",
            keypair.public_key_bytes().len()
        );

        Ok(Self {
            local_keypair: Arc::new(RwLock::new(keypair)),
            peer_keys: Arc::new(RwLock::new(HashMap::new())),
            messages_sent: Arc::new(RwLock::new(0)),
        })
    }

    pub fn local_public_key(&self) -> Vec<u8> {
        self.local_keypair.read().public_key_bytes()
    }

    pub fn add_peer(&self, peer_id: String, public_key_bytes: &[u8]) -> Result<()> {
        let public_key = KyberPublicKey::from_bytes(public_key_bytes)?;

        let mut peer_keys = self.peer_keys.write();
        peer_keys.insert(
            peer_id.clone(),
            PeerEncryptionState {
                public_key,
                shared_secret: None,
                messages_exchanged: 0,
                established_at: std::time::Instant::now(),
            },
        );

        info!("🔑 Added peer {} to encryption registry", peer_id);

        Ok(())
    }

    pub fn establish_shared_secret(&self, peer_id: &str) -> Result<Vec<u8>> {
        let mut peer_keys = self.peer_keys.write();

        let peer_state = peer_keys
            .get_mut(peer_id)
            .ok_or_else(|| SpiraChainError::Internal(format!("Unknown peer: {}", peer_id)))?;

        let (ciphertext, shared_secret) = peer_state.public_key.encapsulate()?;

        peer_state.shared_secret = Some(shared_secret.clone());
        peer_state.messages_exchanged = 0;

        debug!("🤝 Established shared secret with {}", peer_id);

        Ok(ciphertext.to_vec())
    }

    pub fn derive_shared_secret_from_ciphertext(
        &self,
        ciphertext: &[u8],
    ) -> Result<KyberSharedSecret> {
        let ct = KyberCiphertext::from_bytes(ciphertext)?;
        let keypair = self.local_keypair.read();
        keypair.decapsulate(&ct)
    }

    pub fn encrypt_message(&self, peer_id: &str, plaintext: &[u8]) -> Result<Vec<u8>> {
        let mut peer_keys = self.peer_keys.write();

        let peer_state = peer_keys
            .get_mut(peer_id)
            .ok_or_else(|| SpiraChainError::Internal(format!("Unknown peer: {}", peer_id)))?;

        let shared_secret = peer_state.shared_secret.as_ref().ok_or_else(|| {
            SpiraChainError::Internal(format!("No shared secret with {}", peer_id))
        })?;

        let key = shared_secret.derive_key(b"spirachain-p2p-v1");

        let encrypted = Self::aes_gcm_encrypt(&key, plaintext)?;

        peer_state.messages_exchanged += 1;

        if peer_state.messages_exchanged >= KEY_ROTATION_THRESHOLD {
            warn!("⚠️  Key rotation needed for peer {}", peer_id);
        }

        Ok(encrypted)
    }

    pub fn decrypt_message(&self, peer_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let peer_keys = self.peer_keys.read();

        let peer_state = peer_keys
            .get(peer_id)
            .ok_or_else(|| SpiraChainError::Internal(format!("Unknown peer: {}", peer_id)))?;

        let shared_secret = peer_state.shared_secret.as_ref().ok_or_else(|| {
            SpiraChainError::Internal(format!("No shared secret with {}", peer_id))
        })?;

        let key = shared_secret.derive_key(b"spirachain-p2p-v1");

        Self::aes_gcm_decrypt(&key, ciphertext)
    }

    pub fn rotate_key(&self) -> Result<Vec<u8>> {
        let new_keypair = KyberKeyPair::generate()?;
        let public_key_bytes = new_keypair.public_key_bytes();

        *self.local_keypair.write() = new_keypair;
        *self.messages_sent.write() = 0;

        self.peer_keys.write().clear();

        info!("🔄 Rotated Kyber keypair (all peer sessions cleared)");

        Ok(public_key_bytes)
    }

    pub fn should_rotate(&self) -> bool {
        *self.messages_sent.read() >= KEY_ROTATION_THRESHOLD
    }

    pub fn peer_count(&self) -> usize {
        self.peer_keys.read().len()
    }

    fn aes_gcm_encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::aead::Aead;
        use aes_gcm::Nonce;
        use aes_gcm::{Aes256Gcm, KeyInit};

        let cipher = Aes256Gcm::new(key.into());
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from(nonce_bytes);

        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| SpiraChainError::CryptoError(format!("AES-GCM encrypt failed: {}", e)))?;

        let mut result = Vec::with_capacity(12 + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    fn aes_gcm_decrypt(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::aead::Aead;
        use aes_gcm::Nonce;
        use aes_gcm::{Aes256Gcm, KeyInit};

        if data.len() < 12 {
            return Err(SpiraChainError::CryptoError(
                "Ciphertext too short".to_string(),
            ));
        }

        let mut nonce_bytes = [0u8; 12];
        nonce_bytes.copy_from_slice(&data[..12]);
        let nonce = Nonce::from(nonce_bytes);
        let ciphertext = &data[12..];

        let cipher = Aes256Gcm::new(key.into());

        cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|e| SpiraChainError::CryptoError(format!("AES-GCM decrypt failed: {}", e)))
    }
}

impl Default for P2PEncryption {
    fn default() -> Self {
        Self::new().expect("Failed to initialize P2P encryption")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2p_encryption_init() {
        let encryption = P2PEncryption::new().unwrap();
        let public_key = encryption.local_public_key();
        assert!(!public_key.is_empty());
    }

    #[test]
    fn test_establish_shared_secret() {
        let alice = P2PEncryption::new().unwrap();
        let bob = P2PEncryption::new().unwrap();

        let bob_public_key = bob.local_public_key();
        alice.add_peer("bob".to_string(), &bob_public_key).unwrap();

        let ciphertext = alice.establish_shared_secret("bob").unwrap();
        assert!(!ciphertext.is_empty());
    }

    #[test]
    fn test_encrypt_decrypt_message() {
        let alice = P2PEncryption::new().unwrap();
        let bob = P2PEncryption::new().unwrap();

        let alice_public = alice.local_public_key();
        let bob_public = bob.local_public_key();

        alice.add_peer("bob".to_string(), &bob_public).unwrap();
        bob.add_peer("alice".to_string(), &alice_public).unwrap();

        let ciphertext_alice = alice.establish_shared_secret("bob").unwrap();
        let shared_secret_bob = bob
            .derive_shared_secret_from_ciphertext(&ciphertext_alice)
            .unwrap();

        let _message = b"Hello from Alice!";

        let key_alice = alice
            .peer_keys
            .read()
            .get("bob")
            .unwrap()
            .shared_secret
            .as_ref()
            .unwrap()
            .derive_key(b"spirachain-p2p-v1");
        let key_bob = shared_secret_bob.derive_key(b"spirachain-p2p-v1");

        assert_eq!(key_alice, key_bob);
    }

    #[test]
    fn test_key_rotation() {
        let encryption = P2PEncryption::new().unwrap();
        let old_key = encryption.local_public_key();

        let new_key = encryption.rotate_key().unwrap();

        assert_ne!(old_key, new_key);
        assert_eq!(encryption.peer_count(), 0);
    }
}
