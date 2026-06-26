---
type: module
path: "redis_cache.rs"
tags: [rust, module]
---

# Module: redis_cache.rs

**File Link:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs)

## Types Defined
- [[CachedTransactionStatus]] (struct)
- [[RedisCache]] (struct)
- [[RedisCacheError]] (enum)

## Standalone Functions
- [[transaction_status_key|transaction_status_key]]
- [[build_client_url|build_client_url]]
- [[insert_userinfo|insert_userinfo]]
- [[percent_encode_userinfo|percent_encode_userinfo]]
- [[test_disabled_cache|test_disabled_cache]]
- [[test_status_key_format|test_status_key_format]]
- [[test_build_client_url_with_password_file|test_build_client_url_with_password_file]]

## Implementation Methods
- [[RedisCache_new|RedisCache::new]] (impl for [[RedisCache]])
- [[RedisCache_is_enabled|RedisCache::is_enabled]] (impl for [[RedisCache]])
- [[RedisCache_get_transaction_status|RedisCache::get_transaction_status]] (impl for [[RedisCache]])
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]] (impl for [[RedisCache]])
- [[RedisCache_get_connection|RedisCache::get_connection]] (impl for [[RedisCache]])
- [[RedisCache_check_rate_limit|RedisCache::check_rate_limit]] (impl for [[RedisCache]])
- [[RedisCache_with_timeout|RedisCache::with_timeout]] (impl for [[RedisCache]])

