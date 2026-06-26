---
type: struct
module: "redis_cache.rs"
tags: [rust, type/struct]
---

# Struct: CachedTransactionStatus

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L34)

## Definition
```rust
pub struct CachedTransactionStatus {
    pub request_id: String,
    pub status: TxStatus,
    pub active_protocol: String,
}
```

## References
- [[TxStatus|TxStatus]]

## Used By
- [[RedisCache_get_transaction_status|RedisCache::get_transaction_status]]
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]]
- [[QueryRoot|QueryRoot]]
- [[QueryRoot_verify_ndid_record|QueryRoot::verify_ndid_record]]
- [[QueryRoot_get_identity|QueryRoot::get_identity]]
- [[submit_kyc|submit::kyc]]

