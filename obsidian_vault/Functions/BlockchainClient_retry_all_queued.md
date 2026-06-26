---
type: function
module: "blockchain.rs"
parent: "BlockchainClient"
tags: [rust, function]
---

# Function: BlockchainClient::retry_all_queued

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L278)
**Impl Block:** [[BlockchainClient]]

## Signature
```rust
pub async fn retry_all_queued(&self)
```

## Implementation
```rust
pub async fn retry_all_queued(&self) {
        let mut to_retry = Vec::new();
        for (k, v) in self.db.iterator(rocksdb::IteratorMode::Start).flatten() {
            if let Ok(tx) = postcard::from_bytes::<BlockchainTransaction>(&v) {
                to_retry.push((k, tx));
            }
        }

        for (k, tx) in to_retry {
            let timeout_dur = Duration::from_secs(self.config.timeout_secs);
            match timeout(timeout_dur, self.send_to_node(&tx)).await {
                Ok(Ok(receipt)) => {
                    info!(tx_id = %tx.tx_id, block = %receipt.block_number, "Retried transaction finalized");
                    crate::metrics::blockchain_retries()
                        .with_label_values(&["Success"])
                        .inc();
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
                    let _ = self.db.delete(&k);
                }
            }
        }
    }
```

## Calls & References
- [[BlockchainError|BlockchainError]]
- [[BlockchainTransaction|BlockchainTransaction]]
- [[blockchain_retries|blockchain::retries]]

## Called By
- [[main|main]]

