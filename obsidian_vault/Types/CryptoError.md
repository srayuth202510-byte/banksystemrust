---
type: enum
module: "crypto.rs"
tags: [rust, type/enum]
---

# Enum: CryptoError

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L23)

## Definition
```rust
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
```

## Used By
- [[P2pError|P2pError]]
- [[IdentityError|IdentityError]]
- [[BlockchainError|BlockchainError]]
- [[From_from|From::from]]
- [[KeyPair_generate|KeyPair::generate]]
- [[KeyPair_from_bytes|KeyPair::from_bytes]]
- [[HsmClient_new|HsmClient::new]]
- [[HsmClient_sign_ed25519|HsmClient::sign_ed25519]]
- [[HsmClient_find_key|HsmClient::find_key]]
- [[sign|sign]]
- [[verify|verify]]

