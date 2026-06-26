---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_production_requires_redis_password_file

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L448)

## Signature
```rust
fn test_production_requires_redis_password_file()
```

## Implementation
```rust
fn test_production_requires_redis_password_file() {
        let mut cfg = AppConfig::default();
        cfg.redis.enabled = true;
        cfg.redis.url = "rediss://redis.example.internal:6379/".into();
        assert!(cfg.validate_with_mode(true).is_err());
    }
```

## Calls & References
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]
- [[AppConfig_validate_with_mode|AppConfig::validate_with_mode]]

