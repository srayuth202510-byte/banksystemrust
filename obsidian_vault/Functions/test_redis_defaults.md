---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_redis_defaults

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L420)

## Signature
```rust
fn test_redis_defaults()
```

## Implementation
```rust
fn test_redis_defaults() {
        let cfg = AppConfig::default();
        assert!(!cfg.redis.enabled);
        assert_eq!(cfg.redis.ttl_secs, 300);
        assert_eq!(cfg.redis.timeout_ms, 200);
    }
```

## Calls & References
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]

