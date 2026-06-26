---
type: function
module: "network/mod.rs"
parent: ""
tags: [rust, function]
---

# Function: connect_with_fallback

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L231)

## Signature
```rust
pub async fn connect_with_fallback(
    addr: &str,
    tls: &TlsContext,
    quic_timeout_ms: u64,
    tcp_timeout_ms: u64,
) -> (NetworkChannel, Protocol)
```

## Implementation
```rust
pub async fn connect_with_fallback(
    addr: &str,
    tls: &TlsContext,
    quic_timeout_ms: u64,
    tcp_timeout_ms: u64,
) -> (NetworkChannel, Protocol) {
    let quic_timeout = Duration::from_millis(quic_timeout_ms);
    let quic_config = match tls.to_quic_client_config() {
        Ok(c) => c,
        Err(e) => {
            warn!(error = %e, "Failed to build QUIC client config, skipping QUIC");
            return fallback_to_tcp(addr, tls, tcp_timeout_ms).await;
        }
    };

    match timeout(quic_timeout, quic_channel::connect_quic(addr, &quic_config)).await {
        Ok(Ok(channel)) => {
            info!(addr = %addr, "Connected via QUIC");
            (channel, Protocol::Quic)
        }
        Ok(Err(e)) => {
            warn!(addr = %addr, error = %e, "QUIC failed, falling back to TCP");
            fallback_to_tcp(addr, tls, tcp_timeout_ms).await
        }
        Err(_) => {
            warn!(addr = %addr, timeout_ms = quic_timeout_ms, "QUIC timeout, falling back to TCP");
            fallback_to_tcp(addr, tls, tcp_timeout_ms).await
        }
    }
}
```

## Calls & References
- [[TlsContext|TlsContext]]
- [[NetworkChannel|NetworkChannel]]
- [[TlsContext_to_quic_client_config|TlsContext::to_quic_client_config]]
- [[connect_quic|connect::quic]]
- [[fallback_to_tcp|fallback::to_tcp]]
- [[Protocol|Protocol]]

## Called By
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]
- [[NetworkChannel_connect|NetworkChannel::connect]]
- [[test_fallback_on_unreachable|test::fallback_on_unreachable]]

