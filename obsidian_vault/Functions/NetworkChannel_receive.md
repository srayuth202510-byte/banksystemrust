---
type: function
module: "network/mod.rs"
parent: "NetworkChannel"
tags: [rust, function]
---

# Function: NetworkChannel::receive

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L153)
**Impl Block:** [[NetworkChannel]]

## Signature
```rust
async fn receive(&self) -> Result<Vec<u8>, NetworkError>
```

## Implementation
```rust
async fn receive(&self) -> Result<Vec<u8>, NetworkError> {
        let stream = self
            .stream
            .as_ref()
            .ok_or_else(|| NetworkError::ConnectionLost("Not connected".into()))?;

        match stream {
            ConnectionStream::Quic { active_recv, .. } => {
                let mut guard = active_recv.lock().await;
                let mut recv_stream = guard.take().ok_or_else(|| {
                    NetworkError::ConnectionLost(
                        "No active receive stream. Call send() first.".into(),
                    )
                })?;

                let buf = recv_stream
                    .read_to_end(65536)
                    .await
                    .map_err(|e| NetworkError::ConnectionLost(e.to_string()))?;

                Ok(buf)
            }
            ConnectionStream::TcpTls(tls_mutex) => {
                let mut tls_stream = tls_mutex.lock().await;
                let mut buf = Vec::new();
                tls_stream
                    .read_to_end(&mut buf)
                    .await
                    .map_err(|e| NetworkError::TcpFailed(e.to_string()))?;

                Ok(buf)
            }
        }
    }
```

## Calls & References
- [[NetworkError|NetworkError]]
- [[ConnectionStream|ConnectionStream]]

## Called By
- [[ConnectionChannel|ConnectionChannel]]

