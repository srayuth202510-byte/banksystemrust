---
type: enum
module: "p2p_quic.rs"
tags: [rust, type/enum]
---

# Enum: P2pError

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L21)

## Definition
```rust
pub enum P2pError {
    #[error("network error: {0}")]
    Network(#[from] network::NetworkError),
    #[error("crypto error: {0}")]
    Crypto(#[from] crypto::CryptoError),
    #[error("peer not found: {0}")]
    PeerNotFound(String),
    #[error("handshake failed: {0}")]
    HandshakeFailed(String),
    #[error("tls error: {0}")]
    TlsError(String),
}
```

## References
- [[NetworkError|NetworkError]]
- [[CryptoError|CryptoError]]
- [[TlsError|TlsError]]

## Used By
- [[P2pNode_send_kyc|P2pNode::send_kyc]]
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]

