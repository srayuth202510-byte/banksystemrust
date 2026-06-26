---
type: function
module: "network/tcp_channel.rs"
parent: ""
tags: [rust, function]
---

# Function: start_tcp_server

**Defined in:** [network/tcp_channel.rs](file:///home/lokis/Documents/banksystemrust/src/network/tcp_channel.rs#L58)

## Signature
```rust
pub async fn start_tcp_server(
    bind_addr: &str,
    tls: &TlsContext,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<(), NetworkError>
```

## Implementation
```rust
pub async fn start_tcp_server(
    bind_addr: &str,
    tls: &TlsContext,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<(), NetworkError> {
    let listener = TcpListener::bind(bind_addr)
        .await
        .map_err(|e| NetworkError::TcpFailed(format!("server bind failed: {e}")))?;

    let server_config = tls
        .to_rustls_server_config()
        .map_err(|e| NetworkError::TlsError(format!("failed to get rustls config: {e}")))?;

    let acceptor = TlsAcceptor::from(Arc::new(server_config));

    info!(addr = %bind_addr, "TCP+TLS fallback server listening");

    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                info!("TCP server shutting down");
                break;
            }
            result = listener.accept() => {
                match result {
                    Ok((stream, remote_addr)) => {
                        let acceptor = acceptor.clone();
                        tokio::spawn(async move {
                            match acceptor.accept(stream).await {
                                Ok(tls_stream) => {
                                    handle_tcp_connection(tls_stream, remote_addr).await;
                                }
                                Err(e) => {
                                    warn!(remote = %remote_addr, error = %e, "TCP+TLS accept failed");
                                }
                            }
                        });
                    }
                    Err(e) => {
                        warn!(error = %e, "TCP accept error");
                    }
                }
            }
        }
    }
    Ok(())
}
```

## Calls & References
- [[NetworkError|NetworkError]]
- [[TlsContext_to_rustls_server_config|TlsContext::to_rustls_server_config]]
- [[TlsContext|TlsContext]]
- [[TlsContext_clone|TlsContext::clone]]
- [[TlsError|TlsError]]
- [[handle_tcp_connection|handle::tcp_connection]]

## Called By
- [[test_send_kyc_fallback|test::send_kyc_fallback]]
- [[main|main]]

