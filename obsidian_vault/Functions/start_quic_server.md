---
type: function
module: "network/quic_channel.rs"
parent: ""
tags: [rust, function]
---

# Function: start_quic_server

**Defined in:** [network/quic_channel.rs](file:///home/lokis/Documents/banksystemrust/src/network/quic_channel.rs#L56)

## Signature
```rust
pub async fn start_quic_server(
    bind_addr: &str,
    config: quinn::ServerConfig,
) -> Result<quinn::Endpoint, NetworkError>
```

## Implementation
```rust
pub async fn start_quic_server(
    bind_addr: &str,
    config: quinn::ServerConfig,
) -> Result<quinn::Endpoint, NetworkError> {
    let addr: SocketAddr = bind_addr
        .parse()
        .map_err(|e| NetworkError::QuicFailed(format!("invalid bind address {bind_addr}: {e}")))?;

    let endpoint = quinn::Endpoint::server(config, addr)
        .map_err(|e| NetworkError::QuicFailed(format!("server bind failed: {e}")))?;

    info!(addr = %bind_addr, "QUIC server listening");
    Ok(endpoint)
}
```

## Calls & References
- [[NetworkError|NetworkError]]
- [[ServerConfig|ServerConfig]]

## Called By
- [[main|main]]

