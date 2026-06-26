---
type: function
module: "redis_cache.rs"
parent: "RedisCache"
tags: [rust, function]
---

# Function: RedisCache::check_rate_limit

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L126)
**Impl Block:** [[RedisCache]]

## Signature
```rust
pub async fn check_rate_limit(&self, ip: &str, limit: u64) -> Result<bool, RedisCacheError>
```

## Implementation
```rust
pub async fn check_rate_limit(&self, ip: &str, limit: u64) -> Result<bool, RedisCacheError> {
        let Some(client) = &self.client else {
            return Ok(true); // Always allow if Redis is disabled (fallback handled in app)
        };

        let key = format!("ndid:ratelimit:{ip}");
        self.with_timeout(async {
            let mut conn = client.get_multiplexed_async_connection().await?;
            let count: u64 = redis::cmd("INCR").arg(&key).query_async(&mut conn).await?;
            if count == 1 {
                let _: () = redis::cmd("EXPIRE")
                    .arg(&key)
                    .arg(1)
                    .query_async(&mut conn)
                    .await?;
            }
            Ok(count <= limit)
        })
        .await
    }
```

## Calls & References
- [[RedisCache_with_timeout|RedisCache::with_timeout]]
- [[RedisCacheError|RedisCacheError]]

