---
type: enum
module: "network/mod.rs"
tags: [rust, type/enum]
---

# Enum: ConnectionStream

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L56)

## Definition
```rust
pub enum ConnectionStream {
    Quic {
        connection: quinn::Connection,
        active_recv: tokio::sync::Mutex<Option<quinn::RecvStream>>,
    },
    TcpTls(Box<tokio::sync::Mutex<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>>),
}
```

## Used By
- [[connect_quic|connect::quic]]
- [[connect_tcp_tls|connect::tcp_tls]]
- [[NetworkChannel|NetworkChannel]]
- [[NetworkChannel_send|NetworkChannel::send]]
- [[NetworkChannel_receive|NetworkChannel::receive]]

