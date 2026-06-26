---
type: function
module: "identity.rs"
parent: ""
tags: [rust, function]
---

# Function: test_anonymize_removes_pii

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L136)

## Signature
```rust
fn test_anonymize_removes_pii()
```

## Implementation
```rust
fn test_anonymize_removes_pii() {
        let kyc = sample_kyc();
        let anon = kyc.anonymize().unwrap();
        assert!(!anon.identity_hash.contains("1234567890123"));
        assert!(!anon.identity_hash.contains("สมชาย"));
    }
```

## Calls & References
- [[sample_kyc|sample::kyc]]

