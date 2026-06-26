---
type: enum
module: "blockchain.rs"
tags: [rust, type/enum]
---

# Enum: BlockchainError

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L19)

## Definition
```rust
pub enum BlockchainError {
    #[error("node unreachable: {0}")]
    NodeUnreachable(String),
    #[error("transaction failed: {0}")]
    TransactionFailed(String),
    #[error("timeout after {0}s")]
    Timeout(u64),
    #[error("consensus not reached")]
    ConsensusFailed,
    #[error("invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("http error: {0}")]
    Http(String),
    #[error("crypto error: {0}")]
    Crypto(#[from] crypto::CryptoError),
    #[error("database error: {0}")]
    DatabaseError(String),
}
```

## References
- [[CryptoError|CryptoError]]

## Used By
- [[BlockchainClient_new|BlockchainClient::new]]
- [[BlockchainClient_create_transaction|BlockchainClient::create_transaction]]
- [[BlockchainClient_submit|BlockchainClient::submit]]
- [[BlockchainClient_send_to_node|BlockchainClient::send_to_node]]
- [[BlockchainClient_retry_all_queued|BlockchainClient::retry_all_queued]]
- [[BlockchainClient_get_transaction_status|BlockchainClient::get_transaction_status]]

