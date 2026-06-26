---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_rate_limit_validation

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L392)

## Signature
```rust
fn test_rate_limit_validation()
```

## Implementation
```rust
fn test_rate_limit_validation() {
        let mut config = AppConfig::default();
        config.server.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());

        let mut config = AppConfig::default();
        config.server.rate_limit.burst = 0;
        assert!(config.validate().is_err());
    }
```

## Calls & References
- [[AppConfig_validate|AppConfig::validate]]
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]

