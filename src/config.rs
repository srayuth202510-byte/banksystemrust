// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

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
    pub hsm_slot: u32,
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
    pub server: ServerConfig,
    pub network: NetworkConfig,
    pub blockchain: BlockchainConfig,
    pub crypto: CryptoConfig,
    pub logging: LoggingConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
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
            },
            blockchain: BlockchainConfig {
                endpoint: "http://127.0.0.1:9933".into(),
                timeout_secs: 5,
                max_retries: 3,
                db_path: Some("data/tx_queue".into()),
            },
            crypto: CryptoConfig {
                hsm_slot: 0,
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
        settings.try_deserialize()
    }
}
