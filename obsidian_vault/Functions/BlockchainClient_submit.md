---
type: function
module: "blockchain.rs"
parent: "BlockchainClient"
tags: [rust, function]
---

# Function: BlockchainClient::submit

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L168)
**Impl Block:** [[BlockchainClient]]

## Signature
```rust
pub async fn submit(
        &self,
        tx: BlockchainTransaction,
    ) -> Result<TransactionReceipt, BlockchainError>
```

## Implementation
```rust
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
```

## Calls & References
- [[TxStatus|TxStatus]]
- [[TransactionReceipt|TransactionReceipt]]
- [[BlockchainError|BlockchainError]]
- [[BlockchainTransaction|BlockchainTransaction]]

## Called By
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]
- [[test_get_transaction_status|test::get_transaction_status]]
- [[submit_kyc|submit::kyc]]

