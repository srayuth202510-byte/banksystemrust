---
type: function
module: "redis_cache.rs"
parent: "RedisCache"
tags: [rust, function]
---

# Function: RedisCache::get_connection

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L113)
**Impl Block:** [[RedisCache]]

## Signature
```rust
async fn get_connection(
        &self,
        client: &redis::Client,
    ) -> Result<redis::aio::MultiplexedConnection, RedisCacheError>
```

## Implementation
```rust
async fn get_connection(
        &self,
        client: &redis::Client,
    ) -> Result<redis::aio::MultiplexedConnection, RedisCacheError> {
        self.with_timeout(client.get_multiplexed_async_connection())
            .await
            .map_err(|e| {
                warn!(error = %e, "Redis connection unavailable");
                e
            })
    }
```

## Calls & References
- [[RedisCache_with_timeout|RedisCache::with_timeout]]
- [[RedisCacheError|RedisCacheError]]

## Called By
- [[RedisCache_get_transaction_status|RedisCache::get_transaction_status]]
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]]

