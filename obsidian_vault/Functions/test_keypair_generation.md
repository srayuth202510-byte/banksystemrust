---
type: function
module: "crypto.rs"
parent: ""
tags: [rust, function]
---

# Function: test_keypair_generation

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L292)

## Signature
```rust
fn test_keypair_generation()
```

## Implementation
```rust
fn test_keypair_generation() {
        let kp = KeyPair::generate().unwrap();
        assert_eq!(kp.public_key.len(), 32);
    }
```

## Calls & References
- [[KeyPair|KeyPair]]
- [[KeyPair_generate|KeyPair::generate]]

