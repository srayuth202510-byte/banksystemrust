---
type: function
module: "blockchain.rs"
parent: ""
tags: [rust, function]
---

# Function: test_get_transaction_status

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L363)

## Signature
```rust
async fn test_get_transaction_status()
```

## Implementation
```rust
async fn test_get_transaction_status() {
        let config = test_config();
        let client = BlockchainClient::new(config).unwrap();
        let kp = KeyPair::generate().unwrap();
        let tx = client
            .create_transaction("hash".into(), "KBANK".into(), &kp)
            .unwrap();
        let tx_id = tx.tx_id.clone();

        assert!(matches!(
            client.get_transaction_status(&tx_id).unwrap(),
            TxStatus::Finalized
        ));

        let _receipt = client.submit(tx).await.unwrap();
        assert!(matches!(
            client.get_transaction_status(&tx_id).unwrap(),
            TxStatus::Queued
        ));
    }
```

## Calls & References
- [[BlockchainClient_create_transaction|BlockchainClient::create_transaction]]
- [[KeyPair_generate|KeyPair::generate]]
- [[BlockchainClient_submit|BlockchainClient::submit]]
- [[BlockchainClient_get_transaction_status|BlockchainClient::get_transaction_status]]
- [[TxStatus|TxStatus]]
- [[test_config|test::config]]
- [[KeyPair|KeyPair]]
- [[test_create_transaction|test::create_transaction]]
- [[BlockchainClient|BlockchainClient]]
- [[BlockchainClient_new|BlockchainClient::new]]

