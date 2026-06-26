---
type: function
module: "network/mod.rs"
parent: "NetworkChannel"
tags: [rust, function]
---

# Function: NetworkChannel::connect

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L91)
**Impl Block:** [[NetworkChannel]]

## Signature
```rust
async fn connect(&self, addr: &str, tls: &TlsContext) -> Result<NetworkChannel, NetworkError>
```

## Implementation
```rust
async fn connect(&self, addr: &str, tls: &TlsContext) -> Result<NetworkChannel, NetworkError> {
        let (channel, _) = connect_with_fallback(addr, tls, 500, 2000).await;
        if channel.stream.is_some() {
            Ok(channel)
        } else {
            Err(NetworkError::BothFailed)
        }
    }
```

## Calls & References
- [[NetworkError|NetworkError]]
- [[NetworkChannel|NetworkChannel]]
- [[TlsContext|TlsContext]]
- [[connect_with_fallback|connect::with_fallback]]

## Called By
- [[connect_tcp_tls|connect::tcp_tls]]
- [[ConnectionChannel|ConnectionChannel]]

