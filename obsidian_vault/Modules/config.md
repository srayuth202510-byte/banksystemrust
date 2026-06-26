---
type: module
path: "config.rs"
tags: [rust, module]
---

# Module: config.rs

**File Link:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs)

## Types Defined
- [[RateLimitConfig]] (struct)
- [[LoadBalancerConfig]] (struct)
- [[ServerConfig]] (struct)
- [[NetworkConfig]] (struct)
- [[BlockchainConfig]] (struct)
- [[CryptoConfig]] (struct)
- [[LoggingConfig]] (struct)
- [[RedisConfig]] (struct)
- [[AppConfig]] (struct)
- [[AppConfigError]] (enum)
- [[LoadBalancerStrategy]] (enum)

## Standalone Functions
- [[default_tcp_timeout|default_tcp_timeout]]
- [[default_redis_ttl_secs|default_redis_ttl_secs]]
- [[default_redis_timeout_ms|default_redis_timeout_ms]]
- [[is_production_mode|is_production_mode]]
- [[test_default_config_validation|test_default_config_validation]]
- [[test_invalid_bank_code|test_invalid_bank_code]]
- [[test_duplicate_ports|test_duplicate_ports]]
- [[test_invalid_blockchain_endpoint|test_invalid_blockchain_endpoint]]
- [[test_tls_cert_key_must_be_paired|test_tls_cert_key_must_be_paired]]
- [[test_hsm_validation|test_hsm_validation]]
- [[test_rate_limit_validation|test_rate_limit_validation]]
- [[test_rate_limit_defaults|test_rate_limit_defaults]]
- [[test_load_balancer_defaults|test_load_balancer_defaults]]
- [[test_redis_defaults|test_redis_defaults]]
- [[test_invalid_redis_url|test_invalid_redis_url]]
- [[test_production_requires_rediss_for_redis|test_production_requires_rediss_for_redis]]
- [[test_production_requires_redis_password_file|test_production_requires_redis_password_file]]

## Implementation Methods
- [[RateLimitConfig_default|RateLimitConfig::default]] (impl for [[RateLimitConfig]])
- [[RedisConfig_default|RedisConfig::default]] (impl for [[RedisConfig]])
- [[AppConfig_default|AppConfig::default]] (impl for [[AppConfig]])
- [[AppConfig_validate|AppConfig::validate]] (impl for [[AppConfig]])
- [[AppConfig_validate_with_mode|AppConfig::validate_with_mode]] (impl for [[AppConfig]])
- [[AppConfig_load|AppConfig::load]] (impl for [[AppConfig]])

