---
type: function
module: "identity.rs"
parent: ""
tags: [rust, function]
---

# Function: validate_identity_hash

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L86)

## Signature
```rust
pub fn validate_identity_hash(kyc: &KycData, expected_hash: &str) -> Result<bool, IdentityError>
```

## Implementation
```rust
pub fn validate_identity_hash(kyc: &KycData, expected_hash: &str) -> Result<bool, IdentityError> {
    let actual_hash = kyc.compute_hash()?;
    use subtle::ConstantTimeEq;
    Ok(actual_hash
        .as_bytes()
        .ct_eq(expected_hash.as_bytes())
        .into())
}
```

## Calls & References
- [[KycData|KycData]]
- [[KycData_compute_hash|KycData::compute_hash]]
- [[IdentityError|IdentityError]]

