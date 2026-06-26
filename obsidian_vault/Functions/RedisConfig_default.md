---
type: function
module: "config.rs"
parent: "RedisConfig"
tags: [rust, function]
---

# Function: RedisConfig::default

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L147)
**Impl Block:** [[RedisConfig]]

## Signature
```rust
fn default() -> Self
```

## Implementation
```rust
fn default() -> Self {
        Self {
            enabled: false,
            url: "redis://127.0.0.1:6379/".into(),
            username: None,
            password_file: None,
            ttl_secs: default_redis_ttl_secs(),
            timeout_ms: default_redis_timeout_ms(),
        }
    }
```

## Calls & References
- [[default_redis_timeout_ms|default::redis_timeout_ms]]
- [[default_redis_ttl_secs|default::redis_ttl_secs]]

## Called By
- [[AppConfig_default|AppConfig::default]]
- [[test_disabled_cache|test::disabled_cache]]

