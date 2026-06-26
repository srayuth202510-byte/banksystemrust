---
type: function
module: "blockchain.rs"
parent: "BlockchainClient"
tags: [rust, function]
---

# Function: BlockchainClient::drain_queue

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L266)
**Impl Block:** [[BlockchainClient]]

## Signature
```rust
pub fn drain_queue(&self) -> Vec<BlockchainTransaction>
```

## Implementation
```rust
pub fn drain_queue(&self) -> Vec<BlockchainTransaction> {
        let mut drained = Vec::new();
        for (k, v) in self.db.iterator(rocksdb::IteratorMode::Start).flatten() {
            if let Ok(tx) = postcard::from_bytes::<BlockchainTransaction>(&v) {
                drained.push(tx);
            }
            let _ = self.db.delete(&k);
        }
        drained
    }
```

## Calls & References
- [[BlockchainTransaction|BlockchainTransaction]]

