---
type: trait
module: "network/mod.rs"
tags: [rust, type/trait]
---

# Trait: ConnectionChannel

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L83)

## Definition
```rust
pub trait ConnectionChannel: Send + Sync {
    async fn connect(&self, addr: &str, tls: &TlsContext) -> Result<NetworkChannel, NetworkError>;
    async fn send(&self, data: &[u8]) -> Result<(), NetworkError>;
    async fn receive(&self) -> Result<Vec<u8>, NetworkError>;
}
```

## References
- [[NetworkError|NetworkError]]
- [[TlsContext|TlsContext]]
- [[NetworkChannel_send|NetworkChannel::send]]
- [[NetworkChannel|NetworkChannel]]
- [[NetworkChannel_connect|NetworkChannel::connect]]
- [[NetworkChannel_receive|NetworkChannel::receive]]

## Used By
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]

