---
type: function
module: "schema.rs"
parent: "QueryRoot"
tags: [rust, function]
---

# Function: QueryRoot::get_identity

**Defined in:** [schema.rs](file:///home/lokis/Documents/banksystemrust/src/schema.rs#L92)
**Impl Block:** [[QueryRoot]]

## Signature
```rust
async fn get_identity(
        &self,
        ctx: &Context<'_>,
        request_id: String,
    ) -> async_graphql::Result<Option<IdentityStatusGql>>
```

## Implementation
```rust
async fn get_identity(
        &self,
        ctx: &Context<'_>,
        request_id: String,
    ) -> async_graphql::Result<Option<IdentityStatusGql>> {
        let redis_cache = ctx.data::<std::sync::Arc<RedisCache>>()?;
        let blockchain_client =
            ctx.data::<std::sync::Arc<crate::blockchain::BlockchainClient>>()?;

        if let Ok(Some(cached)) = redis_cache.get_transaction_status(&request_id).await {
            return Ok(Some(IdentityStatusGql {
                request_id,
                status: get_identity_status_label(&cached.status),
                active_protocol: cached.active_protocol,
            }));
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
                Ok(Some(IdentityStatusGql {
                    request_id,
                    status: status_str,
                    active_protocol: "TCP/TLS".to_string(),
                }))
            }
            Err(_) => Ok(None),
        }
    }
```

## Calls & References
- [[get_identity_status_label|get::identity_status_label]]
- [[BlockchainClient_get_transaction_status|BlockchainClient::get_transaction_status]]
- [[IdentityStatusGql|IdentityStatusGql]]
- [[BlockchainClient|BlockchainClient]]
- [[RedisCache|RedisCache]]
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]]
- [[RedisCache_get_transaction_status|RedisCache::get_transaction_status]]
- [[CachedTransactionStatus|CachedTransactionStatus]]

## Called By
- [[QueryRoot|QueryRoot]]

