// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppConfigError {
    #[error("{0}")]
    Message(String),
    #[error("config error: {0}")]
    Config(#[from] config::ConfigError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_second: u64,
    pub burst: usize,
    pub per_ip_limit: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 1000,
            burst: 2000,
            per_ip_limit: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum LoadBalancerStrategy {
    #[default]
    RoundRobin,
    Fanout,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancerConfig {
    #[serde(default)]
    pub strategy: LoadBalancerStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub graphql_endpoint: String,
    pub graphql_playground: bool,
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub quic_port: u16,
    pub tcp_port: u16,
    pub quic_timeout_ms: u64,
    pub fallback_enabled: bool,
    pub peers: Vec<String>,
    #[serde(default)]
    pub load_balancer: LoadBalancerConfig,
    #[serde(default)]
    pub cert_path: Option<String>,
    #[serde(default)]
    pub key_path: Option<String>,
    #[serde(default)]
    pub ca_cert_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    pub endpoint: String,
    pub timeout_secs: u64,
    pub max_retries: u32,
    pub db_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    #[serde(default)]
    pub hsm_enabled: bool,
    #[serde(default)]
    pub hsm_library_path: Option<String>,
    #[serde(default)]
    pub hsm_slot: Option<u32>,
    #[serde(default)]
    pub hsm_pin: String,
    pub signing_algorithm: String,
    pub encryption_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    #[serde(default)]
    pub enabled: bool,
    pub url: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password_file: Option<PathBuf>,
    #[serde(default = "default_redis_ttl_secs")]
    pub ttl_secs: u64,
    #[serde(default = "default_redis_timeout_ms")]
    pub timeout_ms: u64,
}

const fn default_redis_ttl_secs() -> u64 {
    300
}

const fn default_redis_timeout_ms() -> u64 {
    200
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            url: "redis://127.0.0.1:6379/".into(),
            username: None,
            password_file: None,
            ttl_secs: default_redis_ttl_secs(),
            timeout_ms: default_redis_timeout_ms(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for AppConfig {
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
                hsm_pin: String::new(),
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
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), String> {
        self.validate_with_mode(is_production_mode())
    }

    pub fn validate_with_mode(&self, production_mode: bool) -> Result<(), String> {
        if self.bank_code.trim().is_empty() {
            return Err("bank_code cannot be empty".into());
        }
        if self.server.port == 0 {
            return Err("server.port cannot be 0".into());
        }
        if self.server.rate_limit.requests_per_second == 0 {
            return Err("server.rate_limit.requests_per_second cannot be 0".into());
        }
        if self.server.rate_limit.burst == 0 {
            return Err("server.rate_limit.burst cannot be 0".into());
        }
        if self.network.quic_port == 0 {
            return Err("network.quic_port cannot be 0".into());
        }
        if self.network.tcp_port == 0 {
            return Err("network.tcp_port cannot be 0".into());
        }
        if self.network.cert_path.is_some() ^ self.network.key_path.is_some() {
            return Err("network.cert_path and network.key_path must be provided together".into());
        }
        if self.server.port == self.network.quic_port
            || self.server.port == self.network.tcp_port
            || self.network.quic_port == self.network.tcp_port
        {
            return Err("ports (server, quic, tcp) must be unique".into());
        }
        if !self.blockchain.endpoint.starts_with("http://")
            && !self.blockchain.endpoint.starts_with("https://")
        {
            return Err("blockchain.endpoint must start with http:// or https://".into());
        }
        if self.crypto.hsm_enabled {
            if let Some(ref path) = self.crypto.hsm_library_path {
                if path.trim().is_empty() {
                    return Err(
                        "crypto.hsm_library_path cannot be empty when hsm_enabled is true".into(),
                    );
                }
            } else {
                return Err(
                    "crypto.hsm_library_path must be specified when hsm_enabled is true".into(),
                );
            }
            if self.crypto.hsm_pin.trim().is_empty() {
                return Err("crypto.hsm_pin cannot be empty when hsm_enabled is true".into());
            }
        }
        if self.redis.enabled {
            if !(self.redis.url.starts_with("redis://") || self.redis.url.starts_with("rediss://"))
            {
                return Err("redis.url must start with redis:// or rediss://".into());
            }
            if production_mode && !self.redis.url.starts_with("rediss://") {
                return Err("redis.url must use rediss:// in production".into());
            }
            if production_mode && self.redis.password_file.is_none() {
                return Err("redis.password_file is required in production".into());
            }
            if self
                .redis
                .username
                .as_ref()
                .is_some_and(|username| username.trim().is_empty())
            {
                return Err("redis.username cannot be empty when provided".into());
            }
            if self
                .redis
                .password_file
                .as_ref()
                .is_some_and(|path| path.as_os_str().is_empty())
            {
                return Err("redis.password_file cannot be empty when provided".into());
            }
            if self.redis.ttl_secs == 0 {
                return Err("redis.ttl_secs cannot be 0 when redis is enabled".into());
            }
            if self.redis.timeout_ms == 0 {
                return Err("redis.timeout_ms cannot be 0 when redis is enabled".into());
            }
        }
        Ok(())
    }

    pub fn load(path: Option<PathBuf>) -> Result<Self, AppConfigError> {
        let cfg_path = path.unwrap_or_else(|| PathBuf::from("config/default.toml"));
        let settings = config::Config::builder()
            .add_source(config::File::from(cfg_path).required(false))
            .add_source(
                config::Environment::with_prefix("NDID")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        let config: Self = settings.try_deserialize()?;
        config.validate().map_err(AppConfigError::Message)?;
        Ok(config)
    }
}

fn is_production_mode() -> bool {
    matches!(
        std::env::var("NDID_ENV"),
        Ok(value) if value.eq_ignore_ascii_case("production")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_validation() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_bank_code() {
        let config = AppConfig {
            bank_code: "".into(),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_duplicate_ports() {
        let mut config = AppConfig::default();
        config.network.quic_port = config.server.port;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_blockchain_endpoint() {
        let mut config = AppConfig::default();
        config.blockchain.endpoint = "ftp://invalid-url".into();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_tls_cert_key_must_be_paired() {
        let mut config = AppConfig::default();
        config.network.cert_path = Some("config/certs/bank.crt".into());
        assert!(config.validate().is_err());

        config.network.key_path = Some("config/certs/bank.key".into());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_hsm_validation() {
        let mut config = AppConfig::default();
        config.crypto.hsm_enabled = true;
        assert!(config.validate().is_err());

        config.crypto.hsm_library_path = Some("/path/to/lib".into());
        config.crypto.hsm_pin = "1234".into();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_rate_limit_validation() {
        let mut config = AppConfig::default();
        config.server.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());

        let mut config = AppConfig::default();
        config.server.rate_limit.burst = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_rate_limit_defaults() {
        let rl = RateLimitConfig::default();
        assert_eq!(rl.requests_per_second, 1000);
        assert_eq!(rl.burst, 2000);
        assert_eq!(rl.per_ip_limit, 100);
    }

    #[test]
    fn test_load_balancer_defaults() {
        let cfg = AppConfig::default();
        assert_eq!(
            cfg.network.load_balancer.strategy,
            LoadBalancerStrategy::RoundRobin
        );
    }

    #[test]
    fn test_redis_defaults() {
        let cfg = AppConfig::default();
        assert!(!cfg.redis.enabled);
        assert_eq!(cfg.redis.ttl_secs, 300);
        assert_eq!(cfg.redis.timeout_ms, 200);
    }

    #[test]
    fn test_invalid_redis_url() {
        let mut cfg = AppConfig::default();
        cfg.redis.enabled = true;
        cfg.redis.url = "http://127.0.0.1:6379".into();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_production_requires_rediss_for_redis() {
        let mut cfg = AppConfig::default();
        cfg.redis.enabled = true;
        cfg.redis.url = "redis://redis.example.internal:6379/".into();
        cfg.redis.password_file = Some(PathBuf::from("/run/secrets/redis_password"));
        assert!(cfg.validate_with_mode(true).is_err());

        cfg.redis.url = "rediss://redis.example.internal:6379/".into();
        assert!(cfg.validate_with_mode(true).is_ok());
    }

    #[test]
    fn test_production_requires_redis_password_file() {
        let mut cfg = AppConfig::default();
        cfg.redis.enabled = true;
        cfg.redis.url = "rediss://redis.example.internal:6379/".into();
        assert!(cfg.validate_with_mode(true).is_err());
    }
}
