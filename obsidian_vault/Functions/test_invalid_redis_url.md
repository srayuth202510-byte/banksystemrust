---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_invalid_redis_url

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L428)

## Signature
```rust
fn test_invalid_redis_url()
```

## Implementation
```rust
fn test_invalid_redis_url() {
        let mut cfg = AppConfig::default();
        cfg.redis.enabled = true;
        cfg.redis.url = "http://127.0.0.1:6379".into();
        assert!(cfg.validate().is_err());
    }
```

## Calls & References
- [[AppConfig_validate|AppConfig::validate]]
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]

