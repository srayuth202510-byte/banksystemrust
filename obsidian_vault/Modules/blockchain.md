---
type: module
path: "blockchain.rs"
tags: [rust, module]
---

# Module: blockchain.rs

**File Link:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs)

## Types Defined
- [[BlockchainTransaction]] (struct)
- [[TransactionReceipt]] (struct)
- [[SubstrateRpcRequest]] (struct)
- [[SubstrateRpcResponse]] (struct)
- [[SubstrateRpcError]] (struct)
- [[BlockchainClient]] (struct)
- [[BlockchainError]] (enum)
- [[TxStatus]] (enum)

## Standalone Functions
- [[test_config|test_config]]
- [[test_create_transaction|test_create_transaction]]
- [[test_submit_transaction_queued_on_no_node|test_submit_transaction_queued_on_no_node]]
- [[test_get_transaction_status|test_get_transaction_status]]

## Implementation Methods
- [[BlockchainClient_new|BlockchainClient::new]] (impl for [[BlockchainClient]])
- [[BlockchainClient_create_transaction|BlockchainClient::create_transaction]] (impl for [[BlockchainClient]])
- [[BlockchainClient_submit|BlockchainClient::submit]] (impl for [[BlockchainClient]])
- [[BlockchainClient_send_to_node|BlockchainClient::send_to_node]] (impl for [[BlockchainClient]])
- [[BlockchainClient_queue_len|BlockchainClient::queue_len]] (impl for [[BlockchainClient]])
- [[BlockchainClient_drain_queue|BlockchainClient::drain_queue]] (impl for [[BlockchainClient]])
- [[BlockchainClient_retry_all_queued|BlockchainClient::retry_all_queued]] (impl for [[BlockchainClient]])
- [[BlockchainClient_get_transaction_status|BlockchainClient::get_transaction_status]] (impl for [[BlockchainClient]])

