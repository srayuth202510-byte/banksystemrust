---
type: struct
module: "config.rs"
tags: [rust, type/struct]
---

# Struct: AppConfig

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L161)

## Definition
```rust
pub struct AppConfig {
    pub bank_code: String,
    pub server: ServerConfig,
    pub network: NetworkConfig,
    pub blockchain: BlockchainConfig,
    pub crypto: CryptoConfig,
    pub logging: LoggingConfig,
    #[serde(default)]
    pub redis: RedisConfig,
}
```

## Associated Functions & Methods
- [[AppConfig_default|AppConfig::default]]
- [[AppConfig_validate|AppConfig::validate]]
- [[AppConfig_validate_with_mode|AppConfig::validate_with_mode]]
- [[AppConfig_load|AppConfig::load]]

## References
- [[ServerConfig|ServerConfig]]
- [[LoggingConfig|LoggingConfig]]
- [[BlockchainConfig|BlockchainConfig]]
- [[CryptoConfig|CryptoConfig]]
- [[RedisConfig|RedisConfig]]
- [[NetworkConfig|NetworkConfig]]

## Used By
- [[test_default_config_validation|test::default_config_validation]]
- [[test_invalid_bank_code|test::invalid_bank_code]]
- [[test_duplicate_ports|test::duplicate_ports]]
- [[test_invalid_blockchain_endpoint|test::invalid_blockchain_endpoint]]
- [[test_tls_cert_key_must_be_paired|test::tls_cert_key_must_be_paired]]
- [[test_hsm_validation|test::hsm_validation]]
- [[test_rate_limit_validation|test::rate_limit_validation]]
- [[test_load_balancer_defaults|test::load_balancer_defaults]]
- [[test_redis_defaults|test::redis_defaults]]
- [[test_invalid_redis_url|test::invalid_redis_url]]
- [[test_production_requires_rediss_for_redis|test::production_requires_rediss_for_redis]]
- [[test_production_requires_redis_password_file|test::production_requires_redis_password_file]]
- [[start_quic_server|start::quic_server]]
- [[main|main]]

