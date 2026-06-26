---
type: enum
module: "network/mod.rs"
tags: [rust, type/enum]
---

# Enum: NetworkError

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L23)

## Definition
```rust
pub enum NetworkError {
    #[error("quic connection failed: {0}")]
    QuicFailed(String),
    #[error("tcp connection failed: {0}")]
    TcpFailed(String),
    #[error("both protocols failed")]
    BothFailed,
    #[error("timeout")]
    Timeout,
    #[error("tls error: {0}")]
    TlsError(String),
    #[error("connection lost: {0}")]
    ConnectionLost(String),
}
```

## References
- [[TlsError|TlsError]]

## Used By
- [[P2pError|P2pError]]
- [[P2pNode_send_kyc|P2pNode::send_kyc]]
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]
- [[connect_quic|connect::quic]]
- [[start_quic_server|start::quic_server]]
- [[connect_tcp_tls|connect::tcp_tls]]
- [[start_tcp_server|start::tcp_server]]
- [[ConnectionChannel|ConnectionChannel]]
- [[NetworkChannel_connect|NetworkChannel::connect]]
- [[NetworkChannel_send|NetworkChannel::send]]
- [[NetworkChannel_receive|NetworkChannel::receive]]

