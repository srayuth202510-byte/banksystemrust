---
type: function
module: "network/tcp_channel.rs"
parent: ""
tags: [rust, function]
---

# Function: connect_tcp_tls

**Defined in:** [network/tcp_channel.rs](file:///home/lokis/Documents/banksystemrust/src/network/tcp_channel.rs#L20)

## Signature
```rust
pub async fn connect_tcp_tls(
    addr: &str,
    tls: &TlsContext,
    tcp_timeout_ms: u64,
) -> Result<NetworkChannel, NetworkError>
```

## Implementation
```rust
pub async fn connect_tcp_tls(
    addr: &str,
    tls: &TlsContext,
    tcp_timeout_ms: u64,
) -> Result<NetworkChannel, NetworkError> {
    info!(addr = %addr, "Attempting TCP+TLS connection");

    let connect_timeout = Duration::from_millis(tcp_timeout_ms);
    let tcp = timeout(connect_timeout, TcpStream::connect(addr))
        .await
        .map_err(|_| NetworkError::Timeout)?
        .map_err(|e| NetworkError::TcpFailed(format!("tcp connect failed: {e}")))?;

    let tls_config = tls
        .to_rustls_client_config()
        .map_err(|e| NetworkError::TlsError(e.to_string()))?;

    let connector = TlsConnector::from(Arc::new(tls_config));
    let host = addr.split(':').next().unwrap_or("localhost");
    let server_name = rustls::pki_types::ServerName::try_from(host.to_string())
        .map_err(|_| NetworkError::TlsError("invalid server name".into()))?;

    let tls_stream = connector
        .connect(server_name, tcp)
        .await
        .map_err(|e| NetworkError::TlsError(format!("tls handshake failed: {e}")))?;

    info!(addr = %addr, "TCP+TLS handshake complete");
    Ok(NetworkChannel {
        protocol: Protocol::Tcp,
        addr: addr.to_string(),
        stream: Some(ConnectionStream::TcpTls(Box::new(tokio::sync::Mutex::new(
            tls_stream,
        )))),
    })
}
```

## Calls & References
- [[NetworkError|NetworkError]]
- [[TlsContext|TlsContext]]
- [[TlsContext_to_rustls_client_config|TlsContext::to_rustls_client_config]]
- [[TlsError|TlsError]]
- [[NetworkChannel|NetworkChannel]]
- [[NetworkChannel_connect|NetworkChannel::connect]]
- [[ConnectionStream|ConnectionStream]]
- [[Protocol|Protocol]]

## Called By
- [[fallback_to_tcp|fallback::to_tcp]]

