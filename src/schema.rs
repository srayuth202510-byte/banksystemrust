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
        let blockchain_client =
            ctx.data_unchecked::<std::sync::Arc<crate::blockchain::BlockchainClient>>();

        let (status, proto) = match blockchain_client.get_transaction_status(&request_id) {
            Ok(crate::blockchain::TxStatus::Finalized) => {
                ("Approved".to_string(), "QUIC".to_string())
            }
            Ok(crate::blockchain::TxStatus::Queued) => {
                ("Queued".to_string(), "TCP/TLS".to_string())
            }
            Ok(crate::blockchain::TxStatus::Pending) => {
                ("Pending".to_string(), "TCP/TLS".to_string())
            }
            _ => ("Rejected".to_string(), "None".to_string()),
        };

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
        let blockchain_client =
            ctx.data_unchecked::<std::sync::Arc<crate::blockchain::BlockchainClient>>();

        match blockchain_client.get_transaction_status(&request_id) {
            Ok(status) => {
                let status_str = match status {
                    crate::blockchain::TxStatus::Pending => "Pending".to_string(),
                    crate::blockchain::TxStatus::Finalized => "Finalized".to_string(),
                    crate::blockchain::TxStatus::Failed => "Failed".to_string(),
                    crate::blockchain::TxStatus::Queued => "Queued".to_string(),
                };
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

        let mut p2p_results = Vec::new();
        for peer in p2p_node.peers() {
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
