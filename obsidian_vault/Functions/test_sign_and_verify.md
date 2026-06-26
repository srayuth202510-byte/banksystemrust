---
type: function
module: "crypto.rs"
parent: ""
tags: [rust, function]
---

# Function: test_sign_and_verify

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L298)

## Signature
```rust
fn test_sign_and_verify()
```

## Implementation
```rust
fn test_sign_and_verify() {
        let kp = KeyPair::generate().unwrap();
        let data = b"test transaction data";
        let signed = sign(data, &kp).unwrap();
        assert!(verify(&signed).unwrap());
    }
```

## Calls & References
- [[sign|sign]]
- [[KeyPair|KeyPair]]
- [[KeyPair_generate|KeyPair::generate]]
- [[verify|verify]]

