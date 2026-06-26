---
type: function
module: "blockchain.rs"
parent: "BlockchainClient"
tags: [rust, function]
---

# Function: BlockchainClient::queue_len

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L261)
**Impl Block:** [[BlockchainClient]]

## Signature
```rust
pub fn queue_len(&self) -> usize
```

## Implementation
```rust
pub fn queue_len(&self) -> usize {
        self.db.iterator(rocksdb::IteratorMode::Start).count()
    }
```

## Called By
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]

