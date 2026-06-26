---
type: error_variant
enum: "RedisCacheError"
variant: "Timeout"
tags: [rust, error, error/RedisCacheError]
---

# Error: RedisCacheError::Timeout

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs)

## Log Pattern / Error Message
`redis operation timed out after {0}ms`

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
