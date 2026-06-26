---
type: function
module: "identity.rs"
parent: ""
tags: [rust, function]
---

# Function: test_create_identity_record

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L144)

## Signature
```rust
fn test_create_identity_record()
```

## Implementation
```rust
fn test_create_identity_record() {
        let kyc = sample_kyc();
        let record =
            create_identity_record("req-001".into(), &kyc, "BBL".into(), "QUIC".into()).unwrap();
        assert!(matches!(record.status, IdentityStatus::Pending));
        assert_eq!(record.identity_hash.len(), 64);
    }
```

## Calls & References
- [[sample_kyc|sample::kyc]]
- [[create_identity_record|create::identity_record]]
- [[IdentityStatus|IdentityStatus]]

