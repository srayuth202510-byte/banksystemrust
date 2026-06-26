---
type: function
module: "crypto.rs"
parent: ""
tags: [rust, function]
---

# Function: test_hash_consistency

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L306)

## Signature
```rust
fn test_hash_consistency()
```

## Implementation
```rust
fn test_hash_consistency() {
        let data = b"identity data";
        let h1 = hash(data);
        let h2 = hash(data);
        assert_eq!(h1, h2);
    }
```

## Called By
- [[test_kyc_hash_consistency|test::kyc_hash_consistency]]

