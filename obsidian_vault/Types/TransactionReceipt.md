---
type: struct
module: "blockchain.rs"
tags: [rust, type/struct]
---

# Struct: TransactionReceipt

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L51)

## Definition
```rust
pub struct TransactionReceipt {
    pub tx_id: String,
    pub block_hash: String,
    pub block_number: u64,
    pub status: TxStatus,
}
```

## References
- [[TxStatus|TxStatus]]

## Used By
- [[BlockchainClient_submit|BlockchainClient::submit]]
- [[BlockchainClient_send_to_node|BlockchainClient::send_to_node]]

