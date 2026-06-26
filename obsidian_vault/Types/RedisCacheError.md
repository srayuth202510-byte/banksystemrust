---
type: enum
module: "redis_cache.rs"
tags: [rust, type/enum]
---

# Enum: RedisCacheError

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L21)

## Definition
```rust
pub enum RedisCacheError {
    #[error("redis client error: {0}")]
    Client(String),
    #[error("redis operation timed out after {0}ms")]
    Timeout(u64),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("redis secret load failed: {0}")]
    SecretLoad(String),
}
```

## Used By
- [[RedisCache_new|RedisCache::new]]
- [[RedisCache_get_transaction_status|RedisCache::get_transaction_status]]
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]]
- [[RedisCache_get_connection|RedisCache::get_connection]]
- [[RedisCache_check_rate_limit|RedisCache::check_rate_limit]]
- [[RedisCache_with_timeout|RedisCache::with_timeout]]
- [[build_client_url|build::client_url]]
- [[insert_userinfo|insert::userinfo]]

