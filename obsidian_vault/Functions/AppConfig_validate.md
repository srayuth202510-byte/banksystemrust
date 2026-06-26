---
type: function
module: "config.rs"
parent: "AppConfig"
tags: [rust, function]
---

# Function: AppConfig::validate

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L221)
**Impl Block:** [[AppConfig]]

## Signature
```rust
pub fn validate(&self) -> Result<(), String>
```

## Implementation
```rust
pub fn validate(&self) -> Result<(), String> {
        self.validate_with_mode(is_production_mode())
    }
```

## Calls & References
- [[is_production_mode|is::production_mode]]

## Called By
- [[AppConfig_load|AppConfig::load]]
- [[test_default_config_validation|test::default_config_validation]]
- [[test_invalid_bank_code|test::invalid_bank_code]]
- [[test_duplicate_ports|test::duplicate_ports]]
- [[test_invalid_blockchain_endpoint|test::invalid_blockchain_endpoint]]
- [[test_tls_cert_key_must_be_paired|test::tls_cert_key_must_be_paired]]
- [[test_hsm_validation|test::hsm_validation]]
- [[test_rate_limit_validation|test::rate_limit_validation]]
- [[test_invalid_redis_url|test::invalid_redis_url]]

