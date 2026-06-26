---
type: function
module: "redis_cache.rs"
parent: ""
tags: [rust, function]
---

# Function: test_disabled_cache

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L243)

## Signature
```rust
fn test_disabled_cache()
```

## Implementation
```rust
fn test_disabled_cache() {
        let cache = RedisCache::new(RedisConfig::default()).unwrap();
        assert!(!cache.is_enabled());
    }
```

## Calls & References
- [[RedisCache|RedisCache]]
- [[RedisCache_new|RedisCache::new]]
- [[RedisConfig_default|RedisConfig::default]]
- [[RedisConfig|RedisConfig]]
- [[RedisCache_is_enabled|RedisCache::is_enabled]]

