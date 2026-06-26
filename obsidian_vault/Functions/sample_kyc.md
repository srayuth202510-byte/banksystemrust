---
type: function
module: "identity.rs"
parent: ""
tags: [rust, function]
---

# Function: sample_kyc

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L117)

## Signature
```rust
fn sample_kyc() -> KycData
```

## Implementation
```rust
fn sample_kyc() -> KycData {
        KycData {
            national_id: "1234567890123".into(),
            full_name: "สมชาย ใจดี".into(),
            date_of_birth: "1990-01-01".into(),
            bank_code: "BBL".into(),
            timestamp: 1700000000,
        }
    }
```

## Calls & References
- [[KycData|KycData]]

## Called By
- [[test_kyc_hash_consistency|test::kyc_hash_consistency]]
- [[test_anonymize_removes_pii|test::anonymize_removes_pii]]
- [[test_create_identity_record|test::create_identity_record]]

