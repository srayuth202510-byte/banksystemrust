---
type: struct
module: "blockchain.rs"
tags: [rust, type/struct]
---

# Struct: BlockchainClient

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L90)

## Definition
```rust
pub struct BlockchainClient {
    config: BlockchainConfig,
    db: rocksdb::DB,
    _temp_dir: Option<tempfile::TempDir>,
    http_client: reqwest::Client,
}
```

## Associated Functions & Methods
- [[BlockchainClient_new|BlockchainClient::new]]
- [[BlockchainClient_create_transaction|BlockchainClient::create_transaction]]
- [[BlockchainClient_submit|BlockchainClient::submit]]
- [[BlockchainClient_send_to_node|BlockchainClient::send_to_node]]
- [[BlockchainClient_queue_len|BlockchainClient::queue_len]]
- [[BlockchainClient_drain_queue|BlockchainClient::drain_queue]]
- [[BlockchainClient_retry_all_queued|BlockchainClient::retry_all_queued]]
- [[BlockchainClient_get_transaction_status|BlockchainClient::get_transaction_status]]

## References
- [[BlockchainConfig|BlockchainConfig]]

## Used By
- [[test_create_transaction|test::create_transaction]]
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]
- [[test_get_transaction_status|test::get_transaction_status]]
- [[QueryRoot|QueryRoot]]
- [[QueryRoot_verify_ndid_record|QueryRoot::verify_ndid_record]]
- [[QueryRoot_get_identity|QueryRoot::get_identity]]
- [[submit_kyc|submit::kyc]]
- [[main|main]]

