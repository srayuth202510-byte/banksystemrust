---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_production_requires_rediss_for_redis

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L436)

## Signature
```rust
fn test_production_requires_rediss_for_redis()
```

## Implementation
```rust
fn test_production_requires_rediss_for_redis() {
        let mut cfg = AppConfig::default();
        cfg.redis.enabled = true;
        cfg.redis.url = "redis://redis.example.internal:6379/".into();
        cfg.redis.password_file = Some(PathBuf::from("/run/secrets/redis_password"));
        assert!(cfg.validate_with_mode(true).is_err());

        cfg.redis.url = "rediss://redis.example.internal:6379/".into();
        assert!(cfg.validate_with_mode(true).is_ok());
    }
```

## Calls & References
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]
- [[AppConfig_validate_with_mode|AppConfig::validate_with_mode]]

