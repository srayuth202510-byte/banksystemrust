---
type: struct
module: "identity.rs"
tags: [rust, type/struct]
---

# Struct: KycData

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L51)

## Definition
```rust
pub struct KycData {
    pub national_id: String,
    pub full_name: String,
    pub date_of_birth: String,
    pub bank_code: String,
    pub timestamp: i64,
}
```

## Associated Functions & Methods
- [[KycData_compute_hash|KycData::compute_hash]]
- [[KycData_anonymize|KycData::anonymize]]

## Used By
- [[validate_identity_hash|validate::identity_hash]]
- [[create_identity_record|create::identity_record]]
- [[sample_kyc|sample::kyc]]
- [[submit_kyc|submit::kyc]]

