---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_tls_cert_key_must_be_paired

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L371)

## Signature
```rust
fn test_tls_cert_key_must_be_paired()
```

## Implementation
```rust
fn test_tls_cert_key_must_be_paired() {
        let mut config = AppConfig::default();
        config.network.cert_path = Some("config/certs/bank.crt".into());
        assert!(config.validate().is_err());

        config.network.key_path = Some("config/certs/bank.key".into());
        assert!(config.validate().is_ok());
    }
```

## Calls & References
- [[AppConfig_validate|AppConfig::validate]]
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]

