---
type: community
members: 10
---

# Infrastructure Security

**Members:** 10 nodes

## Members
- [[Approved security-vetted crates]] - concept - SKILL.md
- [[Conflict Resolution (last-writer-wins, CRDT)]] - rationale - project_plan.html
- [[Consensus & Finality (PBFTBABEGRANDPA)]] - concept - project_plan.html
- [[Distributed Transaction Atomicity (saga pattern)]] - concept - project_plan.html
- [[NTP Synchronization]] - rationale - project_plan.html
- [[Network Segmentation]] - rationale - project_plan.html
- [[Rolling Upgrade & Zero-Downtime Deployment]] - rationale - project_plan.html
- [[Stale Certificate  CRL Propagation Delay]] - concept - project_plan.html
- [[Supply Chain Security (cargo vet)]] - rationale - project_plan.html
- [[mTLS + Certificate Pinning]] - concept - project_plan.html

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Infrastructure_Security
SORT file.name ASC
```

## Connections to other communities
- 1 edge to [[_COMMUNITY_Legacy Integration]]

## Top bridge nodes
- [[Distributed Transaction Atomicity (saga pattern)]] - degree 2, connects to 1 community