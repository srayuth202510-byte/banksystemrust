---
type: module
path: "network/mod.rs"
tags: [rust, module]
---

# Module: network/mod.rs

**File Link:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs)

## Types Defined
- [[NetworkChannel]] (struct)
- [[NetworkError]] (enum)
- [[Protocol]] (enum)
- [[ConnectionStream]] (enum)
- [[ConnectionChannel]] (trait)

## Standalone Functions
- [[process_p2p_message|process_p2p_message]]
- [[connect_with_fallback|connect_with_fallback]]
- [[fallback_to_tcp|fallback_to_tcp]]
- [[test_fallback_on_unreachable|test_fallback_on_unreachable]]
- [[test_protocol_display|test_protocol_display]]

## Implementation Methods
- [[Protocol_fmt|Protocol::fmt]] (impl for [[Protocol]])
- [[Protocol_fmt|Protocol::fmt]] (impl for [[Protocol]])
- [[NetworkChannel_connect|NetworkChannel::connect]] (impl for [[NetworkChannel]])
- [[NetworkChannel_send|NetworkChannel::send]] (impl for [[NetworkChannel]])
- [[NetworkChannel_receive|NetworkChannel::receive]] (impl for [[NetworkChannel]])

