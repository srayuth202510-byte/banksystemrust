---
type: struct
module: "redis_cache.rs"
tags: [rust, type/struct]
---

# Struct: RedisCache

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L41)

## Definition
```rust
pub struct RedisCache {
    client: Option<redis::Client>,
    config: RedisConfig,
}
```

## Associated Functions & Methods
- [[RedisCache_new|RedisCache::new]]
- [[RedisCache_is_enabled|RedisCache::is_enabled]]
- [[RedisCache_get_transaction_status|RedisCache::get_transaction_status]]
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]]
- [[RedisCache_get_connection|RedisCache::get_connection]]
- [[RedisCache_check_rate_limit|RedisCache::check_rate_limit]]
- [[RedisCache_with_timeout|RedisCache::with_timeout]]

## References
- [[RedisConfig|RedisConfig]]

## Used By
- [[test_disabled_cache|test::disabled_cache]]
- [[QueryRoot|QueryRoot]]
- [[QueryRoot_verify_ndid_record|QueryRoot::verify_ndid_record]]
- [[QueryRoot_get_identity|QueryRoot::get_identity]]
- [[submit_kyc|submit::kyc]]
- [[RateLimitState|RateLimitState]]
- [[main|main]]

