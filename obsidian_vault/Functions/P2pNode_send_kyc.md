---
type: function
module: "p2p_quic.rs"
parent: "P2pNode"
tags: [rust, function]
---

# Function: P2pNode::send_kyc

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L100)
**Impl Block:** [[P2pNode]]

## Signature
```rust
pub async fn send_kyc(&self, peer_addr: &str, kyc_hash: String) -> Result<Protocol, P2pError>
```

## Implementation
```rust
pub async fn send_kyc(&self, peer_addr: &str, kyc_hash: String) -> Result<Protocol, P2pError> {
        let res = self.send_kyc_inner(peer_addr, kyc_hash).await;
        match &res {
            Ok(_) => {
                crate::metrics::p2p_messages()
                    .with_label_values(&["out", &self.bank_code, "ACK"])
                    .inc();
            }
            Err(e) => {
                let err_label = match e {
                    P2pError::Network(_) => "NetworkError",
                    P2pError::HandshakeFailed(_) => "HandshakeFailed",
                    _ => "Error",
                };
                crate::metrics::p2p_messages()
                    .with_label_values(&["out", &self.bank_code, err_label])
                    .inc();
            }
        }
        res
    }
```

## Calls & References
- [[NetworkError|NetworkError]]
- [[p2p_messages|p2p::messages]]
- [[P2pError|P2pError]]
- [[Protocol|Protocol]]

## Called By
- [[submit_kyc|submit::kyc]]

