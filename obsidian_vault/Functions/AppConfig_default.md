---
type: function
module: "config.rs"
parent: "AppConfig"
tags: [rust, function]
---

# Function: AppConfig::default

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L173)
**Impl Block:** [[AppConfig]]

## Signature
```rust
fn default() -> Self
```

## Implementation
```rust
fn default() -> Self {
        Self {
            bank_code: "BBL".into(),
            server: ServerConfig {
                host: "0.0.0.0".into(),
                port: 8080,
                graphql_endpoint: "/graphql".into(),
                graphql_playground: true,
                rate_limit: RateLimitConfig::default(),
            },
            network: NetworkConfig {
                quic_port: 4433,
                tcp_port: 8443,
                quic_timeout_ms: 500,
                tcp_timeout_ms: 2000,
                fallback_enabled: true,
                peers: Vec::new(),
                load_balancer: LoadBalancerConfig::default(),
                cert_path: None,
                key_path: None,
                ca_cert_path: None,
            },
            blockchain: BlockchainConfig {
                endpoint: "http://127.0.0.1:9933".into(),
                timeout_secs: 5,
                max_retries: 3,
                db_path: Some("data/tx_queue".into()),
            },
            crypto: CryptoConfig {
                hsm_enabled: false,
                hsm_library_path: None,
                hsm_slot: None,
                hsm_pin_file: None,
                signing_algorithm: "ED25519".into(),
                encryption_algorithm: "AES-256-GCM".into(),
            },
            logging: LoggingConfig {
                level: "info".into(),
                format: "json".into(),
                directory: "/var/log/ndid".into(),
            },
            redis: RedisConfig::default(),
        }
    }
```

## Calls & References
- [[LoadBalancerConfig|LoadBalancerConfig]]
- [[LoggingConfig|LoggingConfig]]
- [[ServerConfig|ServerConfig]]
- [[BlockchainConfig|BlockchainConfig]]
- [[RateLimitConfig|RateLimitConfig]]
- [[RateLimitConfig_default|RateLimitConfig::default]]
- [[RedisConfig_default|RedisConfig::default]]
- [[CryptoConfig|CryptoConfig]]
- [[RedisConfig|RedisConfig]]
- [[NetworkConfig|NetworkConfig]]

## Called By
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
- [[main|main]]

