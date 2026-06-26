---
type: function
module: "identity.rs"
parent: "KycData"
tags: [rust, function]
---

# Function: KycData::compute_hash

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L61)
**Impl Block:** [[KycData]]

## Signature
```rust
pub fn compute_hash(&self) -> Result<String, IdentityError>
```

## Implementation
```rust
pub fn compute_hash(&self) -> Result<String, IdentityError> {
        let serialized = serde_json::to_vec(self)
            .map_err(|e| IdentityError::ValidationFailed(format!("serialization failed: {e}")))?;
        let hash = crypto::hash(&serialized);
        Ok(hex::encode(hash))
    }
```

## Calls & References
- [[IdentityError|IdentityError]]

## Called By
- [[validate_identity_hash|validate::identity_hash]]
- [[create_identity_record|create::identity_record]]
- [[submit_kyc|submit::kyc]]

