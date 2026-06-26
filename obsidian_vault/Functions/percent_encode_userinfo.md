---
type: function
module: "redis_cache.rs"
parent: ""
tags: [rust, function]
---

# Function: percent_encode_userinfo

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L220)

## Signature
```rust
fn percent_encode_userinfo(value: &str) -> String
```

## Implementation
```rust
fn percent_encode_userinfo(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for &byte in value.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(byte as char)
            }
            _ => {
                use std::fmt::Write as _;
                let _ = write!(encoded, "%{:02X}", byte);
            }
        }
    }
    encoded
}
```

## Calls & References
- [[fmt|fmt]]

## Called By
- [[build_client_url|build::client_url]]

