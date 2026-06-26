---
type: enum
module: "identity.rs"
tags: [rust, type/enum]
---

# Enum: IdentityError

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L16)

## Definition
```rust
pub enum IdentityError {
    #[error("validation failed: {0}")]
    ValidationFailed(String),
    #[error("hash mismatch: {0}")]
    HashMismatch(String),
    #[error("record not found: {0}")]
    NotFound(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("crypto error: {0}")]
    Crypto(#[from] crypto::CryptoError),
}
```

## References
- [[CryptoError|CryptoError]]

## Used By
- [[KycData_compute_hash|KycData::compute_hash]]
- [[KycData_anonymize|KycData::anonymize]]
- [[validate_identity_hash|validate::identity_hash]]
- [[create_identity_record|create::identity_record]]

