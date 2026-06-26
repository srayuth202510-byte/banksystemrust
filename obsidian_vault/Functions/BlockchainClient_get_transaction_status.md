---
type: function
module: "blockchain.rs"
parent: "BlockchainClient"
tags: [rust, function]
---

# Function: BlockchainClient::get_transaction_status

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L314)
**Impl Block:** [[BlockchainClient]]

## Signature
```rust
pub fn get_transaction_status(&self, tx_id: &str) -> Result<TxStatus, BlockchainError>
```

## Implementation
```rust
pub fn get_transaction_status(&self, tx_id: &str) -> Result<TxStatus, BlockchainError> {
        match self.db.get(tx_id.as_bytes()) {
            Ok(Some(_)) => Ok(TxStatus::Queued),
            Ok(None) => Ok(TxStatus::Finalized),
            Err(e) => Err(BlockchainError::DatabaseError(e.to_string())),
        }
    }
```

## Calls & References
- [[TxStatus|TxStatus]]
- [[BlockchainError|BlockchainError]]

## Called By
- [[test_get_transaction_status|test::get_transaction_status]]
- [[QueryRoot|QueryRoot]]
- [[QueryRoot_verify_ndid_record|QueryRoot::verify_ndid_record]]
- [[QueryRoot_get_identity|QueryRoot::get_identity]]

