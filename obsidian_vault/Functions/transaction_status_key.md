---
type: function
module: "redis_cache.rs"
parent: ""
tags: [rust, function]
---

# Function: transaction_status_key

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L159)

## Signature
```rust
fn transaction_status_key(request_id: &str) -> String
```

## Implementation
```rust
fn transaction_status_key(request_id: &str) -> String {
    format!("ndid:tx_status:{request_id}")
}
```

## Called By
- [[RedisCache_get_transaction_status|RedisCache::get_transaction_status]]
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]]
- [[test_status_key_format|test::status_key_format]]

