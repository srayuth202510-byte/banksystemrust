// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub graphql_endpoint: String,
    pub graphql_playground: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub quic_port: u16,
    pub tcp_port: u16,
    pub quic_timeout_ms: u64,
    pub fallback_enabled: bool,
    pub peers: Vec<String>,
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
pub struct AppConfig {
    pub bank_code: String,
    pub server: ServerConfig,
    pub network: NetworkConfig,
    pub blockchain: BlockchainConfig,
    pub crypto: CryptoConfig,
    pub logging: LoggingConfig,
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
            },
            network: NetworkConfig {
                quic_port: 4433,
                tcp_port: 8443,
                quic_timeout_ms: 500,
                fallback_enabled: true,
                peers: Vec::new(),
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
        }
    }
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.bank_code.trim().is_empty() {
            return Err("bank_code cannot be empty".into());
        }
        if self.server.port == 0 {
            return Err("server.port cannot be 0".into());
        }
        if self.network.quic_port == 0 {
            return Err("network.quic_port cannot be 0".into());
        }
        if self.network.tcp_port == 0 {
            return Err("network.tcp_port cannot be 0".into());
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
        Ok(())
    }

    pub fn load(path: Option<PathBuf>) -> Result<Self, config::ConfigError> {
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
        config.validate().map_err(config::ConfigError::Message)?;
        Ok(config)
    }
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
        let mut config = AppConfig::default();
        config.bank_code = "".into();
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
    fn test_hsm_validation() {
        let mut config = AppConfig::default();
        config.crypto.hsm_enabled = true;
        // Should fail because library path is None and pin is empty
        assert!(config.validate().is_err());

        config.crypto.hsm_library_path = Some("/path/to/lib".into());
        config.crypto.hsm_pin = "1234".into();
        assert!(config.validate().is_ok());
    }
}
