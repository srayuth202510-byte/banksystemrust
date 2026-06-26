---
type: function
module: "network/quic_channel.rs"
parent: ""
tags: [rust, function]
---

# Function: connect_quic

**Defined in:** [network/quic_channel.rs](file:///home/lokis/Documents/banksystemrust/src/network/quic_channel.rs#L15)

## Signature
```rust
pub async fn connect_quic(
    addr: &str,
    config: &quinn::ClientConfig,
) -> Result<NetworkChannel, NetworkError>
```

## Implementation
```rust
pub async fn connect_quic(
    addr: &str,
    config: &quinn::ClientConfig,
) -> Result<NetworkChannel, NetworkError> {
    info!(addr = %addr, "Attempting QUIC connection");

    let server_addr: SocketAddr = addr
        .parse()
        .map_err(|e| NetworkError::QuicFailed(format!("invalid address {addr}: {e}")))?;

    let local_addr: SocketAddr = if server_addr.is_ipv4() {
        SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), 0)
    } else {
        SocketAddr::new(std::net::IpAddr::V6(std::net::Ipv6Addr::UNSPECIFIED), 0)
    };

    let endpoint = quinn::Endpoint::client(local_addr)
        .map_err(|e| NetworkError::QuicFailed(format!("endpoint creation failed: {e}")))?;

    let server_name = addr.split(':').next().unwrap_or("localhost");

    let connecting = endpoint
        .connect_with(config.clone(), server_addr, server_name)
        .map_err(|e| NetworkError::QuicFailed(format!("connect failed: {e}")))?;

    let connection = connecting
        .await
        .map_err(|e| NetworkError::QuicFailed(format!("handshake failed: {e}")))?;

    info!(addr = %addr, "QUIC handshake complete");
    Ok(NetworkChannel {
        protocol: Protocol::Quic,
        addr: addr.to_string(),
        stream: Some(crate::network::ConnectionStream::Quic {
            connection,
            active_recv: tokio::sync::Mutex::new(None),
        }),
    })
}
```

## Calls & References
- [[NetworkError|NetworkError]]
- [[NetworkChannel|NetworkChannel]]
- [[ConnectionStream|ConnectionStream]]
- [[Protocol|Protocol]]

## Called By
- [[connect_with_fallback|connect::with_fallback]]

