// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use redis::AsyncCommands;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::time::{Duration, timeout};
use tracing::{debug, warn};

use crate::blockchain::TxStatus;
use crate::config::RedisConfig;

#[derive(Debug, Error)]
pub enum RedisCacheError {
    #[error("redis client error: {0}")]
    Client(String),
    #[error("redis operation timed out after {0}ms")]
    Timeout(u64),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("redis secret load failed: {0}")]
    SecretLoad(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTransactionStatus {
    pub request_id: String,
    pub status: TxStatus,
    pub active_protocol: String,
}

#[derive(Debug, Clone)]
pub struct RedisCache {
    client: Option<redis::Client>,
    config: RedisConfig,
}

impl RedisCache {
    pub fn new(config: RedisConfig) -> Result<Self, RedisCacheError> {
        if !config.enabled {
            return Ok(Self {
                client: None,
                config,
            });
        }

        let client_url = build_client_url(&config)?;
        let client =
            redis::Client::open(client_url).map_err(|e| RedisCacheError::Client(e.to_string()))?;

        Ok(Self {
            client: Some(client),
            config,
        })
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.client.is_some()
    }

    pub async fn get_transaction_status(
        &self,
        request_id: &str,
    ) -> Result<Option<CachedTransactionStatus>, RedisCacheError> {
        let Some(client) = &self.client else {
            return Ok(None);
        };

        let mut conn = self.get_connection(client).await?;
        let key = transaction_status_key(request_id);
        let op = conn.get::<_, Option<String>>(&key);
        let value = self.with_timeout(op).await?;

        match value {
            Some(json) => serde_json::from_str(&json)
                .map(Some)
                .map_err(|e| RedisCacheError::Serialization(e.to_string())),
            None => Ok(None),
        }
    }

    pub async fn set_transaction_status(
        &self,
        entry: &CachedTransactionStatus,
    ) -> Result<(), RedisCacheError> {
        let Some(client) = &self.client else {
            return Ok(());
        };

        let payload = serde_json::to_string(entry)
            .map_err(|e| RedisCacheError::Serialization(e.to_string()))?;
        let mut conn = self.get_connection(client).await?;
        let key = transaction_status_key(&entry.request_id);
        let ttl = self.config.ttl_secs;
        let op = conn.set_ex::<_, _, ()>(&key, payload, ttl);
        self.with_timeout(op).await?;
        debug!(request_id = %entry.request_id, "Cached transaction status in Redis");
        Ok(())
    }

    async fn get_connection(
        &self,
        client: &redis::Client,
    ) -> Result<redis::aio::MultiplexedConnection, RedisCacheError> {
        self.with_timeout(client.get_multiplexed_async_connection())
            .await
            .map_err(|e| {
                warn!(error = %e, "Redis connection unavailable");
                e
            })
    }

    async fn with_timeout<F, T>(&self, future: F) -> Result<T, RedisCacheError>
    where
        F: std::future::Future<Output = Result<T, redis::RedisError>>,
    {
        match timeout(Duration::from_millis(self.config.timeout_ms), future).await {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(e)) => Err(RedisCacheError::Client(e.to_string())),
            Err(_) => Err(RedisCacheError::Timeout(self.config.timeout_ms)),
        }
    }
}

fn transaction_status_key(request_id: &str) -> String {
    format!("ndid:tx_status:{request_id}")
}

fn build_client_url(config: &RedisConfig) -> Result<String, RedisCacheError> {
    let Some(password_file) = &config.password_file else {
        return Ok(config.url.clone());
    };

    let password = std::fs::read_to_string(password_file).map_err(|e| {
        RedisCacheError::SecretLoad(format!("cannot read {}: {e}", password_file.display()))
    })?;
    let password = password.trim_end().to_owned();
    if password.is_empty() {
        return Err(RedisCacheError::SecretLoad(
            "redis password file cannot be empty".into(),
        ));
    }
    let password = SecretString::new(password.into());
    let username = config.username.as_deref().unwrap_or("default").trim();
    if username.is_empty() {
        return Err(RedisCacheError::SecretLoad(
            "redis.username cannot be empty when password_file is set".into(),
        ));
    }

    let encoded_password = percent_encode_userinfo(password.expose_secret());
    let auth_segment = if config.username.is_some() {
        format!("{username}:{encoded_password}@")
    } else {
        format!(":{encoded_password}@")
    };

    insert_userinfo(&config.url, &auth_segment)
}

fn insert_userinfo(url: &str, auth_segment: &str) -> Result<String, RedisCacheError> {
    let scheme_pos = url
        .find("://")
        .ok_or_else(|| RedisCacheError::SecretLoad("redis url is missing scheme".into()))?;
    let authority_start = scheme_pos + 3;
    let authority = &url[authority_start..];
    let slash_pos = authority.find('/').unwrap_or(authority.len());
    let authority_prefix = &authority[..slash_pos];
    let authority_suffix = &authority[slash_pos..];

    if authority_prefix.contains('@') {
        return Err(RedisCacheError::SecretLoad(
            "redis url must not contain embedded credentials when password_file is set".into(),
        ));
    }

    Ok(format!(
        "{}{}{}{}",
        &url[..authority_start],
        auth_segment,
        authority_prefix,
        authority_suffix
    ))
}

fn percent_encode_userinfo(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for &byte in value.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(byte as char)
            }
            _ => {
                use std::fmt::Write as _;
                let _ = write!(encoded, "%{:02X}", byte);
            }
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_disabled_cache() {
        let cache = RedisCache::new(RedisConfig::default()).unwrap();
        assert!(!cache.is_enabled());
    }

    #[test]
    fn test_status_key_format() {
        assert_eq!(
            transaction_status_key("tx-123"),
            "ndid:tx_status:tx-123".to_string()
        );
    }

    #[test]
    fn test_build_client_url_with_password_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "s3cr3t!").unwrap();

        let config = RedisConfig {
            enabled: true,
            url: "rediss://redis.example.internal:6379/".into(),
            username: Some("default".into()),
            password_file: Some(file.path().to_path_buf()),
            ttl_secs: 300,
            timeout_ms: 200,
        };

        let url = build_client_url(&config).unwrap();
        assert!(url.starts_with("rediss://default:s3cr3t%21@"));
    }
}
