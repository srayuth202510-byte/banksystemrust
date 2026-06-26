---
type: community
members: 6
---

# Legacy Integration

**Members:** 6 nodes

## Members
- [[Asynchronous Core (tokio runtime)]] - rationale - project_plan.html
- [[Error Semantics Mapping (legacy vs GraphQL)]] - concept - project_plan.html
- [[Identity Schema Divergence (per-bank schemas)]] - concept - project_plan.html
- [[Legacy Core Banking IO bottleneck]] - concept - project_plan.html
- [[Legacy Protocol Translation (ISO 8583SOAPMQ)]] - concept - project_plan.html
- [[Sync-Async Mismatch (blocking vs event-driven)]] - concept - project_plan.html

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Legacy_Integration
SORT file.name ASC
```

## Connections to other communities
- 1 edge to [[_COMMUNITY_System Architecture]]
- 1 edge to [[_COMMUNITY_Infrastructure Security]]

## Top bridge nodes
- [[Sync-Async Mismatch (blocking vs event-driven)]] - degree 4, connects to 1 community
- [[Identity Schema Divergence (per-bank schemas)]] - degree 2, connects to 1 community