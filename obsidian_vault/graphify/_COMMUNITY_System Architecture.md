---
type: community
members: 13
---

# System Architecture

**Members:** 13 nodes

## Members
- [[3-layer system architecture]] - concept - project_plan.html
- [[Bank Node Service]] - concept - AGENTS.md
- [[Blockchain Adapter]] - concept - AGENTS.md
- [[Development trigger words]] - concept - SKILL.md
- [[GraphQL API Gateway]] - concept - AGENTS.md
- [[blockchain.rs (Substrate node client)]] - code - AGENTS.md
- [[crypto.rs (ED25519 signing + AES-GCM encryption)]] - code - AGENTS.md
- [[identity.rs (NDID identity data model + SHA-256)]] - code - AGENTS.md
- [[main.rs (Axum + GraphQL Gateway bootstrap)]] - code - AGENTS.md
- [[p2p_quic.rs (QUIC connection pool + TCP fallback)]] - code - AGENTS.md
- [[quic_channel.rs (Quinn QUIC implementation)]] - code - AGENTS.md
- [[schema.rs (GraphQL Query, Mutation, Subscription)]] - code - AGENTS.md
- [[tcp_channel.rs (Tokio TCP+TLS fallback)]] - code - AGENTS.md

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/System_Architecture
SORT file.name ASC
```

## Connections to other communities
- 1 edge to [[_COMMUNITY_Legacy Integration]]

## Top bridge nodes
- [[identity.rs (NDID identity data model + SHA-256)]] - degree 3, connects to 1 community