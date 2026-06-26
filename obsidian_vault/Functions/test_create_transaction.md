---
type: function
module: "blockchain.rs"
parent: ""
tags: [rust, function]
---

# Function: test_create_transaction

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L338)

## Signature
```rust
fn test_create_transaction()
```

## Implementation
```rust
fn test_create_transaction() {
        let client = BlockchainClient::new(test_config()).unwrap();
        let kp = KeyPair::generate().unwrap();
        let tx = client
            .create_transaction("abc123hash".into(), "BBL".into(), &kp)
            .unwrap();
        assert_eq!(tx.bank_code, "BBL");
        assert_eq!(tx.identity_hash, "abc123hash");
        assert!(!tx.signature.is_empty());
    }
```

## Calls & References
- [[BlockchainClient_create_transaction|BlockchainClient::create_transaction]]
- [[KeyPair_generate|KeyPair::generate]]
- [[test_config|test::config]]
- [[KeyPair|KeyPair]]
- [[BlockchainClient|BlockchainClient]]
- [[BlockchainClient_new|BlockchainClient::new]]

## Called By
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]
- [[test_get_transaction_status|test::get_transaction_status]]

