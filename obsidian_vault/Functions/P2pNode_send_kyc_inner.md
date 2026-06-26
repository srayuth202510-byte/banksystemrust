---
type: function
module: "p2p_quic.rs"
parent: "P2pNode"
tags: [rust, function]
---

# Function: P2pNode::send_kyc_inner

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L122)
**Impl Block:** [[P2pNode]]

## Signature
```rust
async fn send_kyc_inner(
        &self,
        peer_addr: &str,
        kyc_hash: String,
    ) -> Result<Protocol, P2pError>
```

## Implementation
```rust
async fn send_kyc_inner(
        &self,
        peer_addr: &str,
        kyc_hash: String,
    ) -> Result<Protocol, P2pError> {
        info!(from = %self.bank_code, to = %peer_addr, "Sending KYC data");
        let payload = format!("KYC:{}:{}", self.bank_code, kyc_hash);
        let signed = crypto::sign(payload.as_bytes(), &self.keypair)?;

        let message = P2pMessage {
            from_bank: self.bank_code.clone(),
            to_bank: String::new(),
            payload: payload.into_bytes(),
            signature: signed.signature,
            public_key: signed.public_key,
            timestamp: chrono::Utc::now().timestamp(),
        };

        let msg_bytes = serde_json::to_vec(&message)
            .map_err(|e| P2pError::Network(network::NetworkError::TlsError(e.to_string())))?;

        let (channel, protocol) = network::connect_with_fallback(
            peer_addr,
            &self.tls,
            self.quic_timeout_ms,
            self.tcp_timeout_ms,
        )
        .await;
        if channel.stream.is_none() {
            return Err(P2pError::Network(network::NetworkError::BothFailed));
        }

        use crate::network::ConnectionChannel;
        channel.send(&msg_bytes).await?;
        let resp_bytes = channel.receive().await?;
        let resp_str = String::from_utf8_lossy(&resp_bytes);

        if resp_str.starts_with("ERROR:") {
            return Err(P2pError::HandshakeFailed(resp_str.into_owned()));
        }

        info!(protocol = %protocol, response = %resp_str, "KYC sent and ACK received");
        Ok(protocol)
    }
```

## Calls & References
- [[NetworkError|NetworkError]]
- [[P2pError|P2pError]]
- [[TlsError|TlsError]]
- [[P2pMessage|P2pMessage]]
- [[ConnectionChannel|ConnectionChannel]]
- [[Protocol|Protocol]]
- [[sign|sign]]
- [[connect_with_fallback|connect::with_fallback]]

