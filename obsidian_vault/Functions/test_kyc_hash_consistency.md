---
type: function
module: "identity.rs"
parent: ""
tags: [rust, function]
---

# Function: test_kyc_hash_consistency

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L128)

## Signature
```rust
fn test_kyc_hash_consistency()
```

## Implementation
```rust
fn test_kyc_hash_consistency() {
        let kyc = sample_kyc();
        let hash1 = kyc.compute_hash().unwrap();
        let hash2 = kyc.compute_hash().unwrap();
        assert_eq!(hash1, hash2);
    }
```

## Calls & References
- [[sample_kyc|sample::kyc]]
- [[test_hash_consistency|test::hash_consistency]]

