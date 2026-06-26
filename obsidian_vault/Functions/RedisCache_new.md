---
type: function
module: "redis_cache.rs"
parent: "RedisCache"
tags: [rust, function]
---

# Function: RedisCache::new

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L48)
**Impl Block:** [[RedisCache]]

## Signature
```rust
pub fn new(config: RedisConfig) -> Result<Self, RedisCacheError>
```

## Implementation
```rust
pub fn new(config: RedisConfig) -> Result<Self, RedisCacheError> {
        if !config.enabled {
            return Ok(Self {
                client: None,
                config,
            });
        }

        let client_url = build_client_url(&config)?;
        let client =
            redis::Client::open(client_url).map_err(|e| RedisCacheError::Client(e.to_string()))?;

        Ok(Self {
            client: Some(client),
            config,
        })
    }
```

## Calls & References
- [[build_client_url|build::client_url]]
- [[RedisCacheError|RedisCacheError]]
- [[RedisConfig|RedisConfig]]

## Called By
- [[build_client_url|build::client_url]]
- [[test_disabled_cache|test::disabled_cache]]
- [[submit_kyc|submit::kyc]]
- [[main|main]]

