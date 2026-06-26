---
type: struct
module: "crypto.rs"
tags: [rust, type/struct]
---

# Struct: KeyPair

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L42)

## Definition
```rust
pub struct KeyPair {
    pub public_key: Vec<u8>, // กุญแจสาธารณะ 32 ไบต์
    #[serde(skip)]
    secret_key: Vec<u8>, // กุญแจส่วนตัว (ไม่ถูก serialize เพื่อความปลอดภัย)
}
```

## Associated Functions & Methods
- [[KeyPair_drop|KeyPair::drop]]
- [[KeyPair_generate|KeyPair::generate]]
- [[KeyPair_from_bytes|KeyPair::from_bytes]]

## Used By
- [[P2pNode|P2pNode]]
- [[P2pNode_new|P2pNode::new]]
- [[test_node|test::node]]
- [[BlockchainClient_create_transaction|BlockchainClient::create_transaction]]
- [[test_create_transaction|test::create_transaction]]
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]
- [[test_get_transaction_status|test::get_transaction_status]]
- [[sign|sign]]
- [[test_keypair_generation|test::keypair_generation]]
- [[test_sign_and_verify|test::sign_and_verify]]
- [[main|main]]
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]

