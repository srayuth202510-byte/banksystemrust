---
type: enum
module: "network/mod.rs"
tags: [rust, type/enum]
---

# Enum: Protocol

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L40)

## Definition
```rust
pub enum Protocol {
    Quic, // QUIC (0-RTT) - โปรโตคอลหลักความเร็วสูง
    Tcp,  // TCP + TLS 1.3 - ตัวสำรองเมื่อ QUIC ไม่พร้อมใช้งาน
}
```

## Associated Functions & Methods
- [[Protocol_fmt|Protocol::fmt]]
- [[Protocol_fmt|Protocol::fmt]]

## Used By
- [[P2pNode_send_kyc|P2pNode::send_kyc]]
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]
- [[test_send_kyc_fallback|test::send_kyc_fallback]]
- [[connect_quic|connect::quic]]
- [[connect_tcp_tls|connect::tcp_tls]]
- [[NetworkChannel|NetworkChannel]]
- [[Protocol_fmt|Protocol::fmt]]
- [[connect_with_fallback|connect::with_fallback]]
- [[fallback_to_tcp|fallback::to_tcp]]
- [[test_fallback_on_unreachable|test::fallback_on_unreachable]]
- [[test_protocol_display|test::protocol_display]]

