---
type: enum
module: "blockchain.rs"
tags: [rust, type/enum]
---

# Enum: TxStatus

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L60)

## Definition
```rust
pub enum TxStatus {
    Pending,
    Finalized,
    Failed,
    Queued,
}
```

## Used By
- [[TransactionReceipt|TransactionReceipt]]
- [[BlockchainClient_submit|BlockchainClient::submit]]
- [[BlockchainClient_send_to_node|BlockchainClient::send_to_node]]
- [[BlockchainClient_get_transaction_status|BlockchainClient::get_transaction_status]]
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]
- [[test_get_transaction_status|test::get_transaction_status]]
- [[CachedTransactionStatus|CachedTransactionStatus]]
- [[QueryRoot|QueryRoot]]
- [[QueryRoot_verify_ndid_record|QueryRoot::verify_ndid_record]]
- [[submit_kyc|submit::kyc]]
- [[get_identity_status_label|get::identity_status_label]]
- [[verify_status_label|verify::status_label]]

