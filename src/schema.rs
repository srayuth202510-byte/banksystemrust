// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use async_graphql::{Context, Object, SimpleObject};
use tracing::info;

use crate::identity;
use crate::p2p_quic::P2pNode;
use crate::redis_cache::{CachedTransactionStatus, RedisCache};

#[derive(SimpleObject, Clone)]
pub struct IdentityStatusGql {
    pub request_id: String,
    pub status: String,
    pub active_protocol: String,
}

#[derive(SimpleObject, Clone)]
pub struct KycResponse {
    pub request_id: String,
    pub identity_hash: String,
    pub bank_code: String,
    pub message: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn verify_ndid_record(&self, ctx: &Context<'_>, request_id: String) -> IdentityStatusGql {
        let redis_cache = ctx.data_unchecked::<std::sync::Arc<RedisCache>>();
        let blockchain_client =
            ctx.data_unchecked::<std::sync::Arc<crate::blockchain::BlockchainClient>>();

        if let Ok(Some(cached)) = redis_cache.get_transaction_status(&request_id).await {
            return IdentityStatusGql {
                request_id,
                status: verify_status_label(&cached.status),
                active_protocol: cached.active_protocol,
            };
        }

        let (tx_status, proto) = match blockchain_client.get_transaction_status(&request_id) {
            Ok(crate::blockchain::TxStatus::Finalized) => {
                (crate::blockchain::TxStatus::Finalized, "QUIC".to_string())
            }
            Ok(crate::blockchain::TxStatus::Queued) => {
                (crate::blockchain::TxStatus::Queued, "TCP/TLS".to_string())
            }
            Ok(crate::blockchain::TxStatus::Pending) => {
                (crate::blockchain::TxStatus::Pending, "TCP/TLS".to_string())
            }
            _ => (crate::blockchain::TxStatus::Failed, "None".to_string()),
        };

        let status = match tx_status {
            crate::blockchain::TxStatus::Finalized => "Approved".to_string(),
            crate::blockchain::TxStatus::Queued => "Queued".to_string(),
            crate::blockchain::TxStatus::Pending => "Pending".to_string(),
            crate::blockchain::TxStatus::Failed => "Rejected".to_string(),
        };

        let _ = redis_cache
            .set_transaction_status(&CachedTransactionStatus {
                request_id: request_id.clone(),
                status: tx_status,
                active_protocol: proto.clone(),
            })
            .await;

        IdentityStatusGql {
            request_id,
            status,
            active_protocol: proto,
        }
    }

    async fn get_identity(
        &self,
        ctx: &Context<'_>,
        request_id: String,
    ) -> Option<IdentityStatusGql> {
        let redis_cache = ctx.data_unchecked::<std::sync::Arc<RedisCache>>();
        let blockchain_client =
            ctx.data_unchecked::<std::sync::Arc<crate::blockchain::BlockchainClient>>();

        if let Ok(Some(cached)) = redis_cache.get_transaction_status(&request_id).await {
            return Some(IdentityStatusGql {
                request_id,
                status: get_identity_status_label(&cached.status),
                active_protocol: cached.active_protocol,
            });
        }

        match blockchain_client.get_transaction_status(&request_id) {
            Ok(status) => {
                let status_str = get_identity_status_label(&status);
                let _ = redis_cache
                    .set_transaction_status(&CachedTransactionStatus {
                        request_id: request_id.clone(),
                        status,
                        active_protocol: "TCP/TLS".to_string(),
                    })
                    .await;
                Some(IdentityStatusGql {
                    request_id,
                    status: status_str,
                    active_protocol: "TCP/TLS".to_string(),
                })
            }
            Err(_) => None,
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn submit_kyc(
        &self,
        ctx: &Context<'_>,
        national_id: String,
        full_name: String,
        bank_code: String,
    ) -> KycResponse {
        let p2p_node = ctx.data_unchecked::<P2pNode>();
        let redis_cache = ctx.data_unchecked::<std::sync::Arc<RedisCache>>();
        let blockchain_client =
            ctx.data_unchecked::<std::sync::Arc<crate::blockchain::BlockchainClient>>();

        let kyc = identity::KycData {
            national_id,
            full_name,
            date_of_birth: String::new(),
            bank_code: bank_code.clone(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        let identity_hash = match kyc.compute_hash() {
            Ok(h) => h,
            Err(e) => {
                crate::metrics::kyc_requests()
                    .with_label_values(&[&bank_code, "Failed"])
                    .inc();
                return KycResponse {
                    request_id: String::new(),
                    identity_hash: String::new(),
                    bank_code,
                    message: format!("Failed to compute identity hash: {}", e),
                };
            }
        };
        info!(hash = %identity_hash, "KYC submitted");

        let tx = match blockchain_client.create_transaction(
            identity_hash.clone(),
            bank_code.clone(),
            &p2p_node.keypair,
        ) {
            Ok(tx) => tx,
            Err(e) => {
                crate::metrics::kyc_requests()
                    .with_label_values(&[&bank_code, "Failed"])
                    .inc();
                return KycResponse {
                    request_id: String::new(),
                    identity_hash,
                    bank_code,
                    message: format!("Failed to create blockchain tx: {}", e),
                };
            }
        };

        let tx_id = tx.tx_id.clone();

        let receipt = match blockchain_client.submit(tx).await {
            Ok(r) => r,
            Err(e) => {
                crate::metrics::kyc_requests()
                    .with_label_values(&[&bank_code, "Failed"])
                    .inc();
                return KycResponse {
                    request_id: tx_id,
                    identity_hash,
                    bank_code,
                    message: format!("Failed to submit transaction: {}", e),
                };
            }
        };

        let selected_peers = p2p_node.select_peers();
        let mut p2p_results = Vec::new();
        for peer in &selected_peers {
            match p2p_node.send_kyc(peer, identity_hash.clone()).await {
                Ok(proto) => p2p_results.push(format!("Synced with {} via {}", peer, proto)),
                Err(e) => p2p_results.push(format!("Failed to sync with {}: {}", peer, e)),
            }
        }

        let p2p_summary = if p2p_results.is_empty() {
            String::new()
        } else {
            format!("; P2P sync: {}", p2p_results.join(", "))
        };

        let status_str = match receipt.status {
            crate::blockchain::TxStatus::Finalized => "Finalized",
            crate::blockchain::TxStatus::Queued => "Queued",
            _ => "Unknown",
        };
        crate::metrics::kyc_requests()
            .with_label_values(&[&bank_code, status_str])
            .inc();

        let _ = redis_cache
            .set_transaction_status(&CachedTransactionStatus {
                request_id: tx_id.clone(),
                status: receipt.status.clone(),
                active_protocol: if p2p_results.is_empty() {
                    "None".to_string()
                } else {
                    "P2P".to_string()
                },
            })
            .await;

        KycResponse {
            request_id: tx_id,
            identity_hash,
            bank_code,
            message: format!(
                "KYC submitted successfully (Status: {}){}",
                status_str, p2p_summary
            ),
        }
    }
}

fn get_identity_status_label(status: &crate::blockchain::TxStatus) -> String {
    match status {
        crate::blockchain::TxStatus::Pending => "Pending".to_string(),
        crate::blockchain::TxStatus::Finalized => "Finalized".to_string(),
        crate::blockchain::TxStatus::Failed => "Failed".to_string(),
        crate::blockchain::TxStatus::Queued => "Queued".to_string(),
    }
}

fn verify_status_label(status: &crate::blockchain::TxStatus) -> String {
    match status {
        crate::blockchain::TxStatus::Finalized => "Approved".to_string(),
        crate::blockchain::TxStatus::Queued => "Queued".to_string(),
        crate::blockchain::TxStatus::Pending => "Pending".to_string(),
        crate::blockchain::TxStatus::Failed => "Rejected".to_string(),
    }
}
