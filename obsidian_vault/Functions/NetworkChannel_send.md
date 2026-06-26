---
type: function
module: "network/mod.rs"
parent: "NetworkChannel"
tags: [rust, function]
---

# Function: NetworkChannel::send

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L100)
**Impl Block:** [[NetworkChannel]]

## Signature
```rust
async fn send(&self, data: &[u8]) -> Result<(), NetworkError>
```

## Implementation
```rust
async fn send(&self, data: &[u8]) -> Result<(), NetworkError> {
        let stream = self
            .stream
            .as_ref()
            .ok_or_else(|| NetworkError::ConnectionLost("Not connected".into()))?;

        match stream {
            ConnectionStream::Quic {
                connection,
                active_recv,
            } => {
                let (mut send_stream, recv_stream) = connection
                    .open_bi()
                    .await
                    .map_err(|e| NetworkError::ConnectionLost(e.to_string()))?;

                send_stream
                    .write_all(data)
                    .await
                    .map_err(|e| NetworkError::ConnectionLost(e.to_string()))?;

                send_stream
                    .finish()
                    .map_err(|e| NetworkError::ConnectionLost(e.to_string()))?;

                let mut guard = active_recv.lock().await;
                *guard = Some(recv_stream);

                Ok(())
            }
            ConnectionStream::TcpTls(tls_mutex) => {
                let mut tls_stream = tls_mutex.lock().await;

                tls_stream
                    .write_all(data)
                    .await
                    .map_err(|e| NetworkError::TcpFailed(e.to_string()))?;

                tls_stream
                    .flush()
                    .await
                    .map_err(|e| NetworkError::TcpFailed(e.to_string()))?;

                tls_stream
                    .shutdown()
                    .await
                    .map_err(|e| NetworkError::TcpFailed(e.to_string()))?;

                Ok(())
            }
        }
    }
```

## Calls & References
- [[NetworkError|NetworkError]]
- [[ConnectionStream|ConnectionStream]]

## Called By
- [[ConnectionChannel|ConnectionChannel]]

