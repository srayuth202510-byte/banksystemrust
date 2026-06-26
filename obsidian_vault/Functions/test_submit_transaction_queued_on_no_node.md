---
type: function
module: "blockchain.rs"
parent: ""
tags: [rust, function]
---

# Function: test_submit_transaction_queued_on_no_node

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L350)

## Signature
```rust
async fn test_submit_transaction_queued_on_no_node()
```

## Implementation
```rust
async fn test_submit_transaction_queued_on_no_node() {
        let config = test_config();
        let client = BlockchainClient::new(config).unwrap();
        let kp = KeyPair::generate().unwrap();
        let tx = client
            .create_transaction("hash".into(), "SCB".into(), &kp)
            .unwrap();
        let receipt = client.submit(tx).await.unwrap();
        assert!(matches!(receipt.status, TxStatus::Queued));
        assert_eq!(client.queue_len(), 1);
    }
```

## Calls & References
- [[test_node|test::node]]
- [[BlockchainClient_queue_len|BlockchainClient::queue_len]]
- [[BlockchainClient_create_transaction|BlockchainClient::create_transaction]]
- [[KeyPair_generate|KeyPair::generate]]
- [[BlockchainClient_submit|BlockchainClient::submit]]
- [[TxStatus|TxStatus]]
- [[test_config|test::config]]
- [[KeyPair|KeyPair]]
- [[test_create_transaction|test::create_transaction]]
- [[BlockchainClient|BlockchainClient]]
- [[BlockchainClient_new|BlockchainClient::new]]

