---
type: function
module: "redis_cache.rs"
parent: "RedisCache"
tags: [rust, function]
---

# Function: RedisCache::with_timeout

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L147)
**Impl Block:** [[RedisCache]]

## Signature
```rust
async fn with_timeout<F, T>(&self, future: F) -> Result<T, RedisCacheError>
    where
        F: std::future::Future<Output = Result<T, redis::RedisError>>,
```

## Implementation
```rust
async fn with_timeout<F, T>(&self, future: F) -> Result<T, RedisCacheError>
    where
        F: std::future::Future<Output = Result<T, redis::RedisError>>,
    {
        match timeout(Duration::from_millis(self.config.timeout_ms), future).await {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(e)) => Err(RedisCacheError::Client(e.to_string())),
            Err(_) => Err(RedisCacheError::Timeout(self.config.timeout_ms)),
        }
    }
```

## Calls & References
- [[RedisCacheError|RedisCacheError]]

## Called By
- [[RedisCache_get_transaction_status|RedisCache::get_transaction_status]]
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]]
- [[RedisCache_get_connection|RedisCache::get_connection]]
- [[RedisCache_check_rate_limit|RedisCache::check_rate_limit]]

