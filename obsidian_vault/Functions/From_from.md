---
type: function
module: "crypto.rs"
parent: "From"
tags: [rust, function]
---

# Function: From::from

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L49)
**Impl Block:** [[From]]

## Signature
```rust
fn from(e: ed25519_dalek::SignatureError) -> Self
```

## Implementation
```rust
fn from(e: ed25519_dalek::SignatureError) -> Self {
        CryptoError::VerificationFailed(e.to_string())
    }
```

## Calls & References
- [[CryptoError|CryptoError]]

