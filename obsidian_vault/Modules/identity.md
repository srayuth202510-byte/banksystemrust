---
type: module
path: "identity.rs"
tags: [rust, module]
---

# Module: identity.rs

**File Link:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs)

## Types Defined
- [[IdentityRecord]] (struct)
- [[KycData]] (struct)
- [[AnonymizedKyc]] (struct)
- [[IdentityError]] (enum)
- [[IdentityStatus]] (enum)

## Standalone Functions
- [[validate_identity_hash|validate_identity_hash]]
- [[create_identity_record|create_identity_record]]
- [[sample_kyc|sample_kyc]]
- [[test_kyc_hash_consistency|test_kyc_hash_consistency]]
- [[test_anonymize_removes_pii|test_anonymize_removes_pii]]
- [[test_create_identity_record|test_create_identity_record]]

## Implementation Methods
- [[KycData_compute_hash|KycData::compute_hash]] (impl for [[KycData]])
- [[KycData_anonymize|KycData::anonymize]] (impl for [[KycData]])

