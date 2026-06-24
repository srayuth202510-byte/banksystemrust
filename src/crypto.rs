// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use zeroize::Zeroize;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("signing failed: {0}")]
    SigningFailed(String),
    #[error("verification failed: {0}")]
    VerificationFailed(String),
    #[error("encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("key generation failed: {0}")]
    KeyGenerationFailed(String),
    #[error("invalid key: {0}")]
    InvalidKey(String),
    #[error("hsm error: {0}")]
    HsmError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    #[serde(skip)]
    secret_key: Vec<u8>,
}

impl From<ed25519_dalek::SignatureError> for CryptoError {
    fn from(e: ed25519_dalek::SignatureError) -> Self {
        CryptoError::VerificationFailed(e.to_string())
    }
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        self.secret_key.zeroize();
        self.public_key.zeroize();
    }
}

impl KeyPair {
    pub fn generate() -> Result<Self, CryptoError> {
        let mut secret_bytes = [0u8; 32];
        use rand::RngCore;
        OsRng.fill_bytes(&mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        secret_bytes.zeroize();
        let verifying_key = signing_key.verifying_key();
        Ok(Self {
            public_key: verifying_key.to_bytes().to_vec(),
            secret_key: signing_key.to_bytes().to_vec(),
        })
    }

    pub fn from_bytes(secret: &[u8], public: &[u8]) -> Result<Self, CryptoError> {
        let _signing = SigningKey::from_bytes(
            secret
                .try_into()
                .map_err(|_| CryptoError::InvalidKey("invalid secret key length".into()))?,
        );
        let _verifying = VerifyingKey::from_bytes(
            public
                .try_into()
                .map_err(|_| CryptoError::InvalidKey("invalid public key length".into()))?,
        );
        Ok(Self {
            public_key: public.to_vec(),
            secret_key: secret.to_vec(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedPayload {
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

pub fn sign(payload: &[u8], keypair: &KeyPair) -> Result<SignedPayload, CryptoError> {
    let signing_key = SigningKey::from_bytes(
        keypair
            .secret_key
            .as_slice()
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("invalid secret key".into()))?,
    );
    let signature = signing_key.sign(payload);
    Ok(SignedPayload {
        payload: payload.to_vec(),
        signature: signature.to_bytes().to_vec(),
        public_key: keypair.public_key.clone(),
    })
}

pub fn verify(signed: &SignedPayload) -> Result<bool, CryptoError> {
    let verifying_key = VerifyingKey::from_bytes(
        signed
            .public_key
            .as_slice()
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("invalid public key".into()))?,
    )?;
    let signature = Signature::from_slice(&signed.signature)
        .map_err(|e| CryptoError::VerificationFailed(e.to_string()))?;
    Ok(verifying_key.verify(&signed.payload, &signature).is_ok())
}

pub fn hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn hash_hex(data: &[u8]) -> String {
    hex::encode(hash(data))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPayload {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<EncryptedPayload, CryptoError> {
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
    Ok(EncryptedPayload {
        ciphertext,
        nonce: nonce_bytes.to_vec(),
    })
}

pub fn decrypt(encrypted: &EncryptedPayload, key: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
    let nonce = Nonce::from_slice(&encrypted.nonce);
    cipher
        .decrypt(nonce, encrypted.ciphertext.as_ref())
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let kp = KeyPair::generate().unwrap();
        assert_eq!(kp.public_key.len(), 32);
    }

    #[test]
    fn test_sign_and_verify() {
        let kp = KeyPair::generate().unwrap();
        let data = b"test transaction data";
        let signed = sign(data, &kp).unwrap();
        assert!(verify(&signed).unwrap());
    }

    #[test]
    fn test_hash_consistency() {
        let data = b"identity data";
        let h1 = hash(data);
        let h2 = hash(data);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key: [u8; 32] = rand::random();
        let plain = b"sensitive identity data";
        let encrypted = encrypt(plain, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();
        assert_eq!(plain.to_vec(), decrypted);
    }

    #[test]
    fn test_hash_hex_format() {
        let data = b"test";
        let hex_str = hash_hex(data);
        assert_eq!(hex_str.len(), 64);
        assert!(hex_str.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
