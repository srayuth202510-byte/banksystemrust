---
type: struct
module: "blockchain.rs"
tags: [rust, type/struct]
---

# Struct: SubstrateRpcRequest

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L68)

## Definition
```rust
pub struct SubstrateRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub id: u64,
}
```

## Used By
- [[BlockchainClient_send_to_node|BlockchainClient::send_to_node]]

