---
type: struct
module: "network/mod.rs"
tags: [rust, type/struct]
---

# Struct: NetworkChannel

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L75)

## Definition
```rust
pub struct NetworkChannel {
    pub protocol: Protocol,               // โปรโตคอลที่ใช้งาน (Quic/Tcp)
    pub addr: String,                     // ที่อยู่ของ peer
    pub stream: Option<ConnectionStream>, // สตรีมเชื่อมต่อ (None = ไม่ได้เชื่อมต่อ)
}
```

## Associated Functions & Methods
- [[NetworkChannel_connect|NetworkChannel::connect]]
- [[NetworkChannel_send|NetworkChannel::send]]
- [[NetworkChannel_receive|NetworkChannel::receive]]

## References
- [[ConnectionStream|ConnectionStream]]
- [[Protocol|Protocol]]

## Used By
- [[connect_quic|connect::quic]]
- [[connect_tcp_tls|connect::tcp_tls]]
- [[ConnectionChannel|ConnectionChannel]]
- [[NetworkChannel_connect|NetworkChannel::connect]]
- [[connect_with_fallback|connect::with_fallback]]
- [[fallback_to_tcp|fallback::to_tcp]]

