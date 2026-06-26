---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_default_config_validation

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L342)

## Signature
```rust
fn test_default_config_validation()
```

## Implementation
```rust
fn test_default_config_validation() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
    }
```

## Calls & References
- [[AppConfig_validate|AppConfig::validate]]
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]

