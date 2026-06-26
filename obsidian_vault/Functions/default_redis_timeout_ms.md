---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: default_redis_timeout_ms

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L142)

## Signature
```rust
const fn default_redis_timeout_ms() -> u64
```

## Implementation
```rust
const fn default_redis_timeout_ms() -> u64 {
    200
}
```

## Called By
- [[RedisConfig_default|RedisConfig::default]]

