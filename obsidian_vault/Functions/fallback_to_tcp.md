---
type: function
module: "network/mod.rs"
parent: ""
tags: [rust, function]
---

# Function: fallback_to_tcp

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L263)

## Signature
```rust
async fn fallback_to_tcp(
    addr: &str,
    tls: &TlsContext,
    tcp_timeout_ms: u64,
) -> (NetworkChannel, Protocol)
```

## Implementation
```rust
async fn fallback_to_tcp(
    addr: &str,
    tls: &TlsContext,
    tcp_timeout_ms: u64,
) -> (NetworkChannel, Protocol) {
    match tcp_channel::connect_tcp_tls(addr, tls, tcp_timeout_ms).await {
        Ok(channel) => {
            info!(addr = %addr, "Connected via TCP+TLS fallback");
            (channel, Protocol::Tcp)
        }
        Err(e) => {
            warn!(addr = %addr, error = %e, "TCP fallback also failed");
            (
                NetworkChannel {
                    protocol: Protocol::Tcp,
                    addr: addr.to_string(),
                    stream: None,
                },
                Protocol::Tcp,
            )
        }
    }
}
```

## Calls & References
- [[connect_tcp_tls|connect::tcp_tls]]
- [[NetworkChannel|NetworkChannel]]
- [[TlsContext|TlsContext]]
- [[Protocol|Protocol]]

## Called By
- [[connect_with_fallback|connect::with_fallback]]

