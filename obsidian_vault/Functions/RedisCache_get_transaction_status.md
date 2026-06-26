---
type: function
module: "redis_cache.rs"
parent: "RedisCache"
tags: [rust, function]
---

# Function: RedisCache::get_transaction_status

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L72)
**Impl Block:** [[RedisCache]]

## Signature
```rust
pub async fn get_transaction_status(
        &self,
        request_id: &str,
    ) -> Result<Option<CachedTransactionStatus>, RedisCacheError>
```

## Implementation
```rust
pub async fn get_transaction_status(
        &self,
        request_id: &str,
    ) -> Result<Option<CachedTransactionStatus>, RedisCacheError> {
        let Some(client) = &self.client else {
            return Ok(None);
        };

        let mut conn = self.get_connection(client).await?;
        let key = transaction_status_key(request_id);
        let op = conn.get::<_, Option<String>>(&key);
        let value = self.with_timeout(op).await?;

        match value {
            Some(json) => serde_json::from_str(&json)
                .map(Some)
                .map_err(|e| RedisCacheError::Serialization(e.to_string())),
            None => Ok(None),
        }
    }
```

## Calls & References
- [[RedisCache_with_timeout|RedisCache::with_timeout]]
- [[RedisCacheError|RedisCacheError]]
- [[transaction_status_key|transaction::status_key]]
- [[RedisCache_get_connection|RedisCache::get_connection]]
- [[CachedTransactionStatus|CachedTransactionStatus]]

## Called By
- [[QueryRoot|QueryRoot]]
- [[QueryRoot_verify_ndid_record|QueryRoot::verify_ndid_record]]
- [[QueryRoot_get_identity|QueryRoot::get_identity]]

