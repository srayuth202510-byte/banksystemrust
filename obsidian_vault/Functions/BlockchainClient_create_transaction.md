---
type: function
module: "blockchain.rs"
parent: "BlockchainClient"
tags: [rust, function]
---

# Function: BlockchainClient::create_transaction

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L147)
**Impl Block:** [[BlockchainClient]]

## Signature
```rust
pub fn create_transaction(
        &self,
        identity_hash: String,
        bank_code: String,
        keypair: &crypto::KeyPair,
    ) -> Result<BlockchainTransaction, BlockchainError>
```

## Implementation
```rust
pub fn create_transaction(
        &self,
        identity_hash: String,
        bank_code: String,
        keypair: &crypto::KeyPair,
    ) -> Result<BlockchainTransaction, BlockchainError> {
        let tx_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().timestamp();
        let payload = format!("{}:{}:{}", tx_id, identity_hash, bank_code);
        let signed = crypto::sign(payload.as_bytes(), keypair)?;
        Ok(BlockchainTransaction {
            tx_id,
            identity_hash,
            bank_code,
            timestamp,
            signature: signed.signature,
            public_key: signed.public_key,
        })
    }
```

## Calls & References
- [[sign|sign]]
- [[KeyPair|KeyPair]]
- [[BlockchainError|BlockchainError]]
- [[BlockchainTransaction|BlockchainTransaction]]

## Called By
- [[test_create_transaction|test::create_transaction]]
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]
- [[test_get_transaction_status|test::get_transaction_status]]
- [[submit_kyc|submit::kyc]]

