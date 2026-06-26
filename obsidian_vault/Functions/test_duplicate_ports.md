---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_duplicate_ports

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L357)

## Signature
```rust
fn test_duplicate_ports()
```

## Implementation
```rust
fn test_duplicate_ports() {
        let mut config = AppConfig::default();
        config.network.quic_port = config.server.port;
        assert!(config.validate().is_err());
    }
```

## Calls & References
- [[AppConfig_validate|AppConfig::validate]]
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]

