---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_invalid_blockchain_endpoint

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L364)

## Signature
```rust
fn test_invalid_blockchain_endpoint()
```

## Implementation
```rust
fn test_invalid_blockchain_endpoint() {
        let mut config = AppConfig::default();
        config.blockchain.endpoint = "ftp://invalid-url".into();
        assert!(config.validate().is_err());
    }
```

## Calls & References
- [[AppConfig_validate|AppConfig::validate]]
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]

