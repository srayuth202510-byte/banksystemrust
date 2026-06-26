---
type: function
module: "redis_cache.rs"
parent: "RedisCache"
tags: [rust, function]
---

# Function: RedisCache::is_enabled

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L67)
**Impl Block:** [[RedisCache]]

## Signature
```rust
pub fn is_enabled(&self) -> bool
```

## Implementation
```rust
pub fn is_enabled(&self) -> bool {
        self.client.is_some()
    }
```

## Called By
- [[test_disabled_cache|test::disabled_cache]]

