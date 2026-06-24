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
use thiserror::Error;
use tokio::time::{timeout, Duration};
use tracing::{error, info, warn};

pub use crate::config::BlockchainConfig;
use crate::crypto;

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("node unreachable: {0}")]
    NodeUnreachable(String),
    #[error("transaction failed: {0}")]
    TransactionFailed(String),
    #[error("timeout after {0}s")]
    Timeout(u64),
    #[error("consensus not reached")]
    ConsensusFailed,
    #[error("invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("http error: {0}")]
    Http(String),
    #[error("crypto error: {0}")]
    Crypto(#[from] crypto::CryptoError),
    #[error("database error: {0}")]
    DatabaseError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainTransaction {
    pub tx_id: String,
    pub identity_hash: String,
    pub bank_code: String,
    pub timestamp: i64,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub tx_id: String,
    pub block_hash: String,
    pub block_number: u64,
    pub status: TxStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TxStatus {
    Pending,
    Finalized,
    Failed,
    Queued,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstrateRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstrateRpcResponse {
    pub jsonrpc: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<SubstrateRpcError>,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstrateRpcError {
    pub code: i32,
    pub message: String,
}

pub struct BlockchainClient {
    config: BlockchainConfig,
    db: rocksdb::DB,
    _temp_dir: Option<tempfile::TempDir>,
    http_client: reqwest::Client,
}

impl BlockchainClient {
    pub fn new(config: BlockchainConfig) -> Result<Self, BlockchainError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(config.timeout_secs + 2))
            .build()
            .unwrap_or_default();
            
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        let (db, _temp_dir) = if let Some(path) = &config.db_path {
            match rocksdb::DB::open(&opts, path) {
                Ok(db) => (db, None),
                Err(e) => {
                    tracing::warn!("Failed to open rocksdb at {}: {}, falling back to temp dir", path, e);
                    let temp = tempfile::tempdir().map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
                    let db = rocksdb::DB::open(&opts, temp.path()).map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
                    (db, Some(temp))
                }
            }
        } else {
            let temp = tempfile::tempdir().map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
            let db = rocksdb::DB::open(&opts, temp.path()).map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
            (db, Some(temp))
        };

        Ok(Self {
            config,
            db,
            _temp_dir,
            http_client,
        })
    }

    pub fn create_transaction(
        &self,
        identity_hash: String,
        bank_code: String,
        keypair: &crypto::KeyPair,
    ) -> Result<BlockchainTransaction, BlockchainError> {
        let tx_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().timestamp();
        let payload = format!("{}:{}:{}", tx_id, identity_hash, bank_code);
        let signed = crypto::sign(payload.as_bytes(), keypair)?;
        Ok(BlockchainTransaction {
            tx_id,
            identity_hash,
            bank_code,
            timestamp,
            signature: signed.signature,
            public_key: signed.public_key,
        })
    }

    pub async fn submit(&self, tx: BlockchainTransaction) -> Result<TransactionReceipt, BlockchainError> {
        let timeout_dur = Duration::from_secs(self.config.timeout_secs);
        match timeout(timeout_dur, self.send_to_node(&tx)).await {
            Ok(Ok(receipt)) => {
                info!(tx_id = %tx.tx_id, block = %receipt.block_number, "Transaction finalized");
                Ok(receipt)
            }
            Ok(Err(BlockchainError::NodeUnreachable(_))) | Err(_) => {
                warn!(tx_id = %tx.tx_id, "Blockchain node unreachable or timeout, queuing");
                let tx_bytes = bincode::serialize(&tx)
                    .map_err(|e| BlockchainError::TransactionFailed(format!("serialization failed: {e}")))?;
                let _ = self.db.put(tx.tx_id.as_bytes(), tx_bytes);
                Ok(TransactionReceipt {
                    tx_id: tx.tx_id,
                    block_hash: String::new(),
                    block_number: 0,
                    status: TxStatus::Queued,
                })
            }
            Ok(Err(e)) => {
                error!(tx_id = %tx.tx_id, error = %e, "Transaction failed");
                Err(e)
            }
        }
    }

    async fn send_to_node(&self, tx: &BlockchainTransaction) -> Result<TransactionReceipt, BlockchainError> {
        let payload = serde_json::to_value(tx)
            .map_err(|e| BlockchainError::TransactionFailed(e.to_string()))?;

        let rpc_req = SubstrateRpcRequest {
            jsonrpc: "2.0".into(),
            method: "author_submitExtrinsic".into(),
            params: vec![payload],
            id: 1,
        };

        match self.http_client
            .post(&self.config.endpoint)
            .json(&rpc_req)
            .send()
            .await
        {
            Ok(resp) => {
                if !resp.status().is_success() {
                    return Err(BlockchainError::Http(format!(
                        "HTTP {}", resp.status()
                    )));
                }
                let rpc_resp: SubstrateRpcResponse = resp.json().await
                    .map_err(|e| BlockchainError::Http(format!("parse failed: {e}")))?;

                if let Some(err) = rpc_resp.error {
                    return Err(BlockchainError::TransactionFailed(format!(
                        "RPC error {}: {}", err.code, err.message
                    )));
                }

                let block_hash = rpc_resp.result
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| crypto::hash_hex(&serde_json::to_vec(tx).unwrap_or_default()));

                Ok(TransactionReceipt {
                    tx_id: tx.tx_id.clone(),
                    block_hash,
                    block_number: 1,
                    status: TxStatus::Finalized,
                })
            }
            Err(e) => {
                if e.is_timeout() || e.is_connect() {
                    Err(BlockchainError::NodeUnreachable(e.to_string()))
                } else {
                    Err(BlockchainError::Http(e.to_string()))
                }
            }
        }
    }

    pub fn queue_len(&self) -> usize {
        self.db.iterator(rocksdb::IteratorMode::Start).count()
    }

    pub fn drain_queue(&self) -> Vec<BlockchainTransaction> {
        let mut drained = Vec::new();
        for item in self.db.iterator(rocksdb::IteratorMode::Start) {
            if let Ok((k, v)) = item {
                if let Ok(tx) = bincode::deserialize::<BlockchainTransaction>(&v) {
                    drained.push(tx);
                }
                let _ = self.db.delete(&k);
            }
        }
        drained
    }

    pub async fn retry_all_queued(&self) {
        let mut to_retry = Vec::new();
        for item in self.db.iterator(rocksdb::IteratorMode::Start) {
            if let Ok((k, v)) = item {
                if let Ok(tx) = bincode::deserialize::<BlockchainTransaction>(&v) {
                    to_retry.push((k, tx));
                }
            }
        }

        for (k, tx) in to_retry {
            let timeout_dur = Duration::from_secs(self.config.timeout_secs);
            match timeout(timeout_dur, self.send_to_node(&tx)).await {
                Ok(Ok(receipt)) => {
                    info!(tx_id = %tx.tx_id, block = %receipt.block_number, "Retried transaction finalized");
                    let _ = self.db.delete(&k);
                }
                Ok(Err(BlockchainError::NodeUnreachable(_))) | Err(_) => {
                    warn!(tx_id = %tx.tx_id, "Retry failed: Blockchain node unreachable or timeout");
                }
                Ok(Err(e)) => {
                    error!(tx_id = %tx.tx_id, error = %e, "Retried transaction failed");
                    let _ = self.db.delete(&k);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::KeyPair;

    fn test_config() -> BlockchainConfig {
        BlockchainConfig {
            endpoint: "http://127.0.0.1:9933".into(),
            timeout_secs: 2,
            max_retries: 3,
            db_path: None,
        }
    }

    #[test]
    fn test_create_transaction() {
        let client = BlockchainClient::new(test_config()).unwrap();
        let kp = KeyPair::generate().unwrap();
        let tx = client.create_transaction(
            "abc123hash".into(),
            "BBL".into(),
            &kp,
        ).unwrap();
        assert_eq!(tx.bank_code, "BBL");
        assert_eq!(tx.identity_hash, "abc123hash");
        assert!(!tx.signature.is_empty());
    }

    #[tokio::test]
    async fn test_submit_transaction_queued_on_no_node() {
        let config = test_config();
        let client = BlockchainClient::new(config).unwrap();
        let kp = KeyPair::generate().unwrap();
        let tx = client.create_transaction("hash".into(), "SCB".into(), &kp).unwrap();
        let receipt = client.submit(tx).await.unwrap();
        assert!(matches!(receipt.status, TxStatus::Queued));
        assert_eq!(client.queue_len(), 1);
    }
}
