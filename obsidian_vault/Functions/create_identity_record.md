---
type: function
module: "identity.rs"
parent: ""
tags: [rust, function]
---

# Function: create_identity_record

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L96)

## Signature
```rust
pub fn create_identity_record(
    request_id: String,
    kyc: &KycData,
    bank_code: String,
    protocol: String,
) -> Result<IdentityRecord, IdentityError>
```

## Implementation
```rust
pub fn create_identity_record(
    request_id: String,
    kyc: &KycData,
    bank_code: String,
    protocol: String,
) -> Result<IdentityRecord, IdentityError> {
    let identity_hash = kyc.compute_hash()?;
    Ok(IdentityRecord {
        request_id,
        status: IdentityStatus::Pending,
        identity_hash,
        timestamp: chrono::Utc::now().timestamp(),
        bank_code,
        active_protocol: protocol,
    })
}
```

## Calls & References
- [[KycData_compute_hash|KycData::compute_hash]]
- [[IdentityRecord|IdentityRecord]]
- [[IdentityError|IdentityError]]
- [[IdentityStatus|IdentityStatus]]
- [[KycData|KycData]]

## Called By
- [[test_create_identity_record|test::create_identity_record]]

