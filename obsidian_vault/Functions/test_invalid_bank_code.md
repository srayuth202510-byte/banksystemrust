---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_invalid_bank_code

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L348)

## Signature
```rust
fn test_invalid_bank_code()
```

## Implementation
```rust
fn test_invalid_bank_code() {
        let config = AppConfig {
            bank_code: "".into(),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }
```

## Calls & References
- [[AppConfig_validate|AppConfig::validate]]
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]

