---
type: function
module: "redis_cache.rs"
parent: "RedisCache"
tags: [rust, function]
---

# Function: RedisCache::set_transaction_status

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L94)
**Impl Block:** [[RedisCache]]

## Signature
```rust
pub async fn set_transaction_status(
        &self,
        entry: &CachedTransactionStatus,
    ) -> Result<(), RedisCacheError>
```

## Implementation
```rust
pub async fn set_transaction_status(
        &self,
        entry: &CachedTransactionStatus,
    ) -> Result<(), RedisCacheError> {
        let Some(client) = &self.client else {
            return Ok(());
        };

        let payload = serde_json::to_string(entry)
            .map_err(|e| RedisCacheError::Serialization(e.to_string()))?;
        let mut conn = self.get_connection(client).await?;
        let key = transaction_status_key(&entry.request_id);
        let ttl = self.config.ttl_secs;
        let op = conn.set_ex::<_, _, ()>(&key, payload, ttl);
        self.with_timeout(op).await?;
        debug!(request_id = %entry.request_id, "Cached transaction status in Redis");
        Ok(())
    }
```

## Calls & References
- [[RedisCache_with_timeout|RedisCache::with_timeout]]
- [[RedisCacheError|RedisCacheError]]
- [[load_key|load::key]]
- [[transaction_status_key|transaction::status_key]]
- [[RedisCache_get_connection|RedisCache::get_connection]]
- [[CachedTransactionStatus|CachedTransactionStatus]]

## Called By
- [[QueryRoot|QueryRoot]]
- [[QueryRoot_verify_ndid_record|QueryRoot::verify_ndid_record]]
- [[QueryRoot_get_identity|QueryRoot::get_identity]]
- [[submit_kyc|submit::kyc]]

