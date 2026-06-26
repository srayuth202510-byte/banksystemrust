---
type: function
module: "blockchain.rs"
parent: ""
tags: [rust, function]
---

# Function: test_config

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L328)

## Signature
```rust
fn test_config() -> BlockchainConfig
```

## Implementation
```rust
fn test_config() -> BlockchainConfig {
        BlockchainConfig {
            endpoint: "http://127.0.0.1:9933".into(),
            timeout_secs: 2,
            max_retries: 3,
            db_path: None,
        }
    }
```

## Calls & References
- [[BlockchainConfig|BlockchainConfig]]

## Called By
- [[test_create_transaction|test::create_transaction]]
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]
- [[test_get_transaction_status|test::get_transaction_status]]
- [[test_quic_configs|test::quic_configs]]

