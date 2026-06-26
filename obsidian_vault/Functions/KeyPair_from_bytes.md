---
type: function
module: "crypto.rs"
parent: "KeyPair"
tags: [rust, function]
---

# Function: KeyPair::from_bytes

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L77)
**Impl Block:** [[KeyPair]]

## Signature
```rust
pub fn from_bytes(secret: &[u8], public: &[u8]) -> Result<Self, CryptoError>
```

## Implementation
```rust
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
```

## Calls & References
- [[CryptoError|CryptoError]]

## Called By
- [[sign|sign]]

