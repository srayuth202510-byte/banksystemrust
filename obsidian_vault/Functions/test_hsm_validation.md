---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_hsm_validation

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L381)

## Signature
```rust
fn test_hsm_validation()
```

## Implementation
```rust
fn test_hsm_validation() {
        let mut config = AppConfig::default();
        config.crypto.hsm_enabled = true;
        assert!(config.validate().is_err());

        config.crypto.hsm_library_path = Some("/path/to/lib".into());
        config.crypto.hsm_pin_file = Some(PathBuf::from("/etc/secrets/pin.txt"));
        assert!(config.validate().is_ok());
    }
```

## Calls & References
- [[AppConfig_validate|AppConfig::validate]]
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]

