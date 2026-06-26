---
type: function
module: "crypto.rs"
parent: ""
tags: [rust, function]
---

# Function: test_hash_hex_format

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L323)

## Signature
```rust
fn test_hash_hex_format()
```

## Implementation
```rust
fn test_hash_hex_format() {
        let data = b"test";
        let hex_str = hash_hex(data);
        assert_eq!(hex_str.len(), 64);
        assert!(hex_str.chars().all(|c| c.is_ascii_hexdigit()));
    }
```

## Calls & References
- [[hash_hex|hash::hex]]

