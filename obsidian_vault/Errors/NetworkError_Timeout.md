---
type: error_variant
enum: "NetworkError"
variant: "Timeout"
tags: [rust, error, error/NetworkError]
---

# Error: NetworkError::Timeout

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs)

## Log Pattern / Error Message
`timeout`

## Functions Generating/Handling this Error
- [[BlockchainClient_retry_all_queued|BlockchainClient::retry_all_queued]]
- [[connect_tcp_tls|connect::tcp_tls]]
- [[RedisCache_with_timeout|RedisCache::with_timeout]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[BlockchainClient_retry_all_queued]].

### Remediation
- How to resolve the error in runtime or code.
