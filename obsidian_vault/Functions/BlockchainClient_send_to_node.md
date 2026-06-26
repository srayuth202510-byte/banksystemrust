---
type: function
module: "blockchain.rs"
parent: "BlockchainClient"
tags: [rust, function]
---

# Function: BlockchainClient::send_to_node

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L199)
**Impl Block:** [[BlockchainClient]]

## Signature
```rust
async fn send_to_node(
        &self,
        tx: &BlockchainTransaction,
    ) -> Result<TransactionReceipt, BlockchainError>
```

## Implementation
```rust
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
```

## Calls & References
- [[TransactionReceipt|TransactionReceipt]]
- [[SubstrateRpcRequest|SubstrateRpcRequest]]
- [[hash_hex|hash::hex]]
- [[TxStatus|TxStatus]]
- [[BlockchainError|BlockchainError]]
- [[SubstrateRpcResponse|SubstrateRpcResponse]]
- [[BlockchainTransaction|BlockchainTransaction]]

