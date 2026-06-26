---
type: struct
module: "blockchain.rs"
tags: [rust, type/struct]
---

# Struct: BlockchainTransaction

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L40)

## Definition
```rust
pub struct BlockchainTransaction {
    pub tx_id: String,
    pub identity_hash: String,
    pub bank_code: String,
    pub timestamp: i64,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}
```

## Used By
- [[BlockchainClient_create_transaction|BlockchainClient::create_transaction]]
- [[BlockchainClient_submit|BlockchainClient::submit]]
- [[BlockchainClient_send_to_node|BlockchainClient::send_to_node]]
- [[BlockchainClient_drain_queue|BlockchainClient::drain_queue]]
- [[BlockchainClient_retry_all_queued|BlockchainClient::retry_all_queued]]

