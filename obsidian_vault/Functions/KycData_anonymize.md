---
type: function
module: "identity.rs"
parent: "KycData"
tags: [rust, function]
---

# Function: KycData::anonymize

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L69)
**Impl Block:** [[KycData]]

## Signature
```rust
pub fn anonymize(&self) -> Result<AnonymizedKyc, IdentityError>
```

## Implementation
```rust
pub fn anonymize(&self) -> Result<AnonymizedKyc, IdentityError> {
        Ok(AnonymizedKyc {
            identity_hash: self.compute_hash()?,
            bank_code: self.bank_code.clone(),
            timestamp: self.timestamp,
        })
    }
```

## Calls & References
- [[IdentityError|IdentityError]]
- [[AnonymizedKyc|AnonymizedKyc]]

