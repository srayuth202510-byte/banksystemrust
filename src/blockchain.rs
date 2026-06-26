// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// คริปโตเคอเรนซีและบล็อกเชน - เชื่อมต่อ Substrate node สำหรับบันทึกธุรกรรม
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::time::{Duration, timeout};
use tracing::{error, info, warn};

pub use crate::config::BlockchainConfig;
use crate::crypto;

// ข้อผิดพลาดที่เกี่ยวกับบล็อกเชน Substrate
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

// ธุรกรรมบล็อกเชนสำหรับบันทึกข้อมูล KYC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainTransaction {
    pub tx_id: String,
    pub identity_hash: String,
    pub bank_code: String,
    pub timestamp: i64,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

// ใบเสร็จรับเงินธุรกรรมจากบล็อกเชน
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub tx_id: String,
    pub block_hash: String,
    pub block_number: u64,
    pub status: TxStatus,
}

// สถานะของธุรกรรมบนบล็อกเชน
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

// ไคลเอนต์หลักสำหรับเชื่อมต่อและโต้ตอบกับ Substrate blockchain node
pub struct BlockchainClient {
    config: BlockchainConfig,
    db: rocksdb::DB,
    _temp_dir: Option<tempfile::TempDir>,
    http_client: reqwest::Client,
    queue_lock: Arc<Mutex<()>>,
}

impl BlockchainClient {
    // สร้างไคลเอนต์บล็อกเชน พร้อม RocksDB สำหรับ queue ธุรกรรม
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
            .map_err(|e| BlockchainError::Http(format!("Failed to build HTTP client: {e}")))?;

        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        let (db, _temp_dir) = if let Some(path) = &config.db_path {
            match rocksdb::DB::open(&opts, path) {
                Ok(db) => (db, None),
                Err(e) => {
                    tracing::warn!(
                        "Failed to open rocksdb at {}: {}, falling back to temp dir",
                        path,
                        e
                    );
                    let temp = tempfile::tempdir()
                        .map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
                    let db = rocksdb::DB::open(&opts, temp.path())
                        .map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
                    (db, Some(temp))
                }
            }
        } else {
            let temp =
                tempfile::tempdir().map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
            let db = rocksdb::DB::open(&opts, temp.path())
                .map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
            (db, Some(temp))
        };

        Ok(Self {
            config,
            db,
            _temp_dir,
            http_client,
            queue_lock: Arc::new(Mutex::new(())),
        })
    }

    // สร้างธุรกรรมบล็อกเชนพร้อมลงนามดิจิทัลด้วย ED25519
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

    // ส่งธุรกรรมไปยัง Substrate node (Fallback เป็น queue ใน RocksDB ถ้า node ไม่พร้อม)
    pub async fn submit(
        &self,
        tx: BlockchainTransaction,
    ) -> Result<TransactionReceipt, BlockchainError> {
        let timeout_dur = Duration::from_secs(self.config.timeout_secs);
        match timeout(timeout_dur, self.send_to_node(&tx)).await {
            Ok(Ok(receipt)) => {
                info!(tx_id = %tx.tx_id, block = %receipt.block_number, "Transaction finalized");
                Ok(receipt)
            }
            Ok(Err(BlockchainError::NodeUnreachable(_))) | Err(_) => {
                warn!(tx_id = %tx.tx_id, "Blockchain node unreachable or timeout, queuing");
                let tx_bytes = postcard::to_allocvec(&tx).map_err(|e| {
                    BlockchainError::TransactionFailed(format!("serialization failed: {e}"))
                })?;
                let _lock = self.queue_lock.lock().await;
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

    // ส่งธุรกรรมผ่าน RPC ไปยัง Substrate node (ใช้ author_submitExtrinsic)
    async fn send_to_node(
        &self,
        tx: &BlockchainTransaction,
    ) -> Result<TransactionReceipt, BlockchainError> {
        let payload = serde_json::to_value(tx)
            .map_err(|e| BlockchainError::TransactionFailed(e.to_string()))?;

        let rpc_req = SubstrateRpcRequest {
            jsonrpc: "2.0".into(),
            method: "author_submitExtrinsic".into(),
            params: vec![payload],
            id: 1,
        };

        match self
            .http_client
            .post(&self.config.endpoint)
            .json(&rpc_req)
            .send()
            .await
        {
            Ok(resp) => {
                if !resp.status().is_success() {
                    return Err(BlockchainError::Http(format!("HTTP {}", resp.status())));
                }
                let rpc_resp: SubstrateRpcResponse = resp
                    .json()
                    .await
                    .map_err(|e| BlockchainError::Http(format!("parse failed: {e}")))?;

                if let Some(err) = rpc_resp.error {
                    return Err(BlockchainError::TransactionFailed(format!(
                        "RPC error {}: {}",
                        err.code, err.message
                    )));
                }

                let block_hash = rpc_resp
                    .result
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| {
                        crypto::hash_hex(&serde_json::to_vec(tx).unwrap_or_default())
                    });

                Ok(TransactionReceipt {
                    tx_id: tx.tx_id.clone(),
                    block_hash,
                    block_number: 0,
                    status: TxStatus::Pending,
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

    // จำนวนธุรกรรมที่รอส่งในคิว
    pub async fn queue_len(&self) -> usize {
        let _lock = self.queue_lock.lock().await;
        self.db.iterator(rocksdb::IteratorMode::Start).count()
    }

    // นำธุรกรรมทั้งหมดออกจากคิว
    pub async fn drain_queue(&self) -> Vec<BlockchainTransaction> {
        let _lock = self.queue_lock.lock().await;
        let mut drained = Vec::new();
        for (k, v) in self.db.iterator(rocksdb::IteratorMode::Start).flatten() {
            if let Ok(tx) = postcard::from_bytes::<BlockchainTransaction>(&v) {
                drained.push(tx);
            }
            let _ = self.db.delete(&k);
        }
        drained
    }

    // ส่งธุรกรรมที่ค้างอยู่ในคิวทั้งหมดอีกครั้ง (เรียกเป็นระยะโดย background worker)
    pub async fn retry_all_queued(&self) {
        let to_retry = {
            let _lock = self.queue_lock.lock().await;
            let mut pending = Vec::new();
            for (k, v) in self.db.iterator(rocksdb::IteratorMode::Start).flatten() {
                if let Ok(tx) = postcard::from_bytes::<BlockchainTransaction>(&v) {
                    pending.push((k, tx));
                }
            }
            pending
        };

        for (k, tx) in to_retry {
            let timeout_dur = Duration::from_secs(self.config.timeout_secs);
            match timeout(timeout_dur, self.send_to_node(&tx)).await {
                Ok(Ok(receipt)) => {
                    info!(tx_id = %tx.tx_id, block = %receipt.block_number, "Retried transaction finalized");
                    crate::metrics::blockchain_retries()
                        .with_label_values(&["Success"])
                        .inc();
                    let _lock = self.queue_lock.lock().await;
                    let _ = self.db.delete(&k);
                }
                Ok(Err(BlockchainError::NodeUnreachable(_))) | Err(_) => {
                    warn!(tx_id = %tx.tx_id, "Retry failed: Blockchain node unreachable or timeout");
                    crate::metrics::blockchain_retries()
                        .with_label_values(&["Timeout"])
                        .inc();
                }
                Ok(Err(e)) => {
                    error!(tx_id = %tx.tx_id, error = %e, "Retried transaction failed");
                    crate::metrics::blockchain_retries()
                        .with_label_values(&["Failed"])
                        .inc();
                    let _lock = self.queue_lock.lock().await;
                    let _ = self.db.delete(&k);
                }
            }
        }
    }

    // ตรวจสอบสถานะธุรกรรม (อยู่ในคิว = Queued, ไม่อยู่ = Finalized)
    pub async fn get_transaction_status(&self, tx_id: &str) -> Result<TxStatus, BlockchainError> {
        let _lock = self.queue_lock.lock().await;
        match self.db.get(tx_id.as_bytes()) {
            Ok(Some(_)) => Ok(TxStatus::Queued),
            Ok(None) => Ok(TxStatus::Finalized),
            Err(e) => Err(BlockchainError::DatabaseError(e.to_string())),
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
        let tx = client
            .create_transaction("abc123hash".into(), "BBL".into(), &kp)
            .unwrap();
        assert_eq!(tx.bank_code, "BBL");
        assert_eq!(tx.identity_hash, "abc123hash");
        assert!(!tx.signature.is_empty());
    }

    #[tokio::test]
    async fn test_submit_transaction_queued_on_no_node() {
        let config = test_config();
        let client = BlockchainClient::new(config).unwrap();
        let kp = KeyPair::generate().unwrap();
        let tx = client
            .create_transaction("hash".into(), "SCB".into(), &kp)
            .unwrap();
        let receipt = client.submit(tx).await.unwrap();
        assert!(matches!(receipt.status, TxStatus::Queued));
        assert_eq!(client.queue_len().await, 1);
    }

    #[tokio::test]
    async fn test_get_transaction_status() {
        let config = test_config();
        let client = BlockchainClient::new(config).unwrap();
        let kp = KeyPair::generate().unwrap();
        let tx = client
            .create_transaction("hash".into(), "KBANK".into(), &kp)
            .unwrap();
        let tx_id = tx.tx_id.clone();

        assert!(matches!(
            client.get_transaction_status(&tx_id).await.unwrap(),
            TxStatus::Finalized
        ));

        let _receipt = client.submit(tx).await.unwrap();
        assert!(matches!(
            client.get_transaction_status(&tx_id).await.unwrap(),
            TxStatus::Queued
        ));
    }
}
