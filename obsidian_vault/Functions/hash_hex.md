---
type: function
module: "crypto.rs"
parent: ""
tags: [rust, function]
---

# Function: hash_hex

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L141)

## Signature
```rust
pub fn hash_hex(data: &[u8]) -> String
```

## Implementation
```rust
pub fn hash_hex(data: &[u8]) -> String {
    hex::encode(hash(data))
}
```

## Called By
- [[BlockchainClient_send_to_node|BlockchainClient::send_to_node]]
- [[test_hash_hex_format|test::hash_hex_format]]

