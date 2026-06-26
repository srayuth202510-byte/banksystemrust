---
type: function
module: "network/tcp_channel.rs"
parent: ""
tags: [rust, function]
---

# Function: handle_tcp_connection

**Defined in:** [network/tcp_channel.rs](file:///home/lokis/Documents/banksystemrust/src/network/tcp_channel.rs#L107)

## Signature
```rust
pub async fn handle_tcp_connection(
    mut tls_stream: tokio_rustls::server::TlsStream<TcpStream>,
    remote: std::net::SocketAddr,
)
```

## Implementation
```rust
pub async fn handle_tcp_connection(
    mut tls_stream: tokio_rustls::server::TlsStream<TcpStream>,
    remote: std::net::SocketAddr,
) {
    info!(remote = %remote, "TCP+TLS connection accepted");

    let mut buf = Vec::new();
    match tls_stream.read_to_end(&mut buf).await {
        Ok(_) => {
            info!(remote = %remote, len = %buf.len(), "TCP data received");
            let response = crate::network::process_p2p_message(&buf);
            if let Err(e) = tls_stream.write_all(response.as_bytes()).await {
                warn!(error = %e, "TCP send response failed");
            }
            if let Err(e) = tls_stream.flush().await {
                warn!(error = %e, "TCP flush failed");
            }
            let _ = tls_stream.shutdown().await;
        }
        Err(e) => {
            warn!(remote = %remote, error = %e, "TCP read error");
        }
    }

    info!(remote = %remote, "TCP+TLS connection closed");
}
```

## Calls & References
- [[process_p2p_message|process::p2p_message]]

## Called By
- [[start_tcp_server|start::tcp_server]]

