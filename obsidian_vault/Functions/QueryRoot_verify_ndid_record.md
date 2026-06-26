---
type: function
module: "schema.rs"
parent: "QueryRoot"
tags: [rust, function]
---

# Function: QueryRoot::verify_ndid_record

**Defined in:** [schema.rs](file:///home/lokis/Documents/banksystemrust/src/schema.rs#L39)
**Impl Block:** [[QueryRoot]]

## Signature
```rust
async fn verify_ndid_record(
        &self,
        ctx: &Context<'_>,
        request_id: String,
    ) -> async_graphql::Result<IdentityStatusGql>
```

## Implementation
```rust
async fn verify_ndid_record(
        &self,
        ctx: &Context<'_>,
        request_id: String,
    ) -> async_graphql::Result<IdentityStatusGql> {
        let redis_cache = ctx.data::<std::sync::Arc<RedisCache>>()?;
        let blockchain_client =
            ctx.data::<std::sync::Arc<crate::blockchain::BlockchainClient>>()?;

        if let Ok(Some(cached)) = redis_cache.get_transaction_status(&request_id).await {
            return Ok(IdentityStatusGql {
                request_id,
                status: verify_status_label(&cached.status),
                active_protocol: cached.active_protocol,
            });
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

        Ok(IdentityStatusGql {
            request_id,
            status,
            active_protocol: proto,
        })
    }
```

## Calls & References
- [[BlockchainClient_get_transaction_status|BlockchainClient::get_transaction_status]]
- [[TxStatus|TxStatus]]
- [[verify_status_label|verify::status_label]]
- [[IdentityStatusGql|IdentityStatusGql]]
- [[BlockchainClient|BlockchainClient]]
- [[RedisCache|RedisCache]]
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]]
- [[RedisCache_get_transaction_status|RedisCache::get_transaction_status]]
- [[CachedTransactionStatus|CachedTransactionStatus]]

## Called By
- [[QueryRoot|QueryRoot]]

