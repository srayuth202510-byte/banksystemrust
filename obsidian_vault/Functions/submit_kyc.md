---
type: function
module: "schema.rs"
parent: ""
tags: [rust, function]
---

# Function: submit_kyc

**Defined in:** [schema.rs](file:///home/lokis/Documents/banksystemrust/src/schema.rs#L136)

## Signature
```rust
async fn submit_kyc(
        &self,
        ctx: &Context<'_>,
        national_id: String,
        full_name: String,
        bank_code: String,
    ) -> async_graphql::Result<KycResponse>
```

## Implementation
```rust
async fn submit_kyc(
        &self,
        ctx: &Context<'_>,
        national_id: String,
        full_name: String,
        bank_code: String,
    ) -> async_graphql::Result<KycResponse> {
        let p2p_node = ctx.data::<P2pNode>()?;
        let redis_cache = ctx.data::<std::sync::Arc<RedisCache>>()?;
        let blockchain_client =
            ctx.data::<std::sync::Arc<crate::blockchain::BlockchainClient>>()?;

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
                return Ok(KycResponse {
                    request_id: String::new(),
                    identity_hash: String::new(),
                    bank_code,
                    message: format!("Failed to compute identity hash: {}", e),
                });
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
                return Ok(KycResponse {
                    request_id: String::new(),
                    identity_hash,
                    bank_code,
                    message: format!("Failed to create blockchain tx: {}", e),
                });
            }
        };

        let tx_id = tx.tx_id.clone();

        let receipt = match blockchain_client.submit(tx).await {
            Ok(r) => r,
            Err(e) => {
                crate::metrics::kyc_requests()
                    .with_label_values(&[&bank_code, "Failed"])
                    .inc();
                return Ok(KycResponse {
                    request_id: tx_id,
                    identity_hash,
                    bank_code,
                    message: format!("Failed to submit transaction: {}", e),
                });
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

        Ok(KycResponse {
            request_id: tx_id,
            identity_hash,
            bank_code,
            message: format!(
                "KYC submitted successfully (Status: {}){}",
                status_str, p2p_summary
            ),
        })
    }
```

## Calls & References
- [[KycData_compute_hash|KycData::compute_hash]]
- [[BlockchainClient_create_transaction|BlockchainClient::create_transaction]]
- [[KycResponse|KycResponse]]
- [[BlockchainClient_submit|BlockchainClient::submit]]
- [[TxStatus|TxStatus]]
- [[P2pNode_select_peers|P2pNode::select_peers]]
- [[P2pNode_send_kyc|P2pNode::send_kyc]]
- [[P2pNode_new|P2pNode::new]]
- [[kyc_requests|kyc::requests]]
- [[P2pNode|P2pNode]]
- [[BlockchainClient|BlockchainClient]]
- [[KycData|KycData]]
- [[RedisCache|RedisCache]]
- [[BlockchainClient_new|BlockchainClient::new]]
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]]
- [[RedisCache_new|RedisCache::new]]
- [[P2pNode_peers|P2pNode::peers]]
- [[CachedTransactionStatus|CachedTransactionStatus]]

