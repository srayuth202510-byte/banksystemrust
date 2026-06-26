---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: default_redis_ttl_secs

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L138)

## Signature
```rust
const fn default_redis_ttl_secs() -> u64
```

## Implementation
```rust
const fn default_redis_ttl_secs() -> u64 {
    300
}
```

## Called By
- [[RedisConfig_default|RedisConfig::default]]

