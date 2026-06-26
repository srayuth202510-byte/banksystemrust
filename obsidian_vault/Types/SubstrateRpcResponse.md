---
type: struct
module: "blockchain.rs"
tags: [rust, type/struct]
---

# Struct: SubstrateRpcResponse

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L76)

## Definition
```rust
pub struct SubstrateRpcResponse {
    pub jsonrpc: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<SubstrateRpcError>,
    pub id: u64,
}
```

## References
- [[SubstrateRpcError|SubstrateRpcError]]

## Used By
- [[BlockchainClient_send_to_node|BlockchainClient::send_to_node]]

