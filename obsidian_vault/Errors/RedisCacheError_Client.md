---
type: error_variant
enum: "RedisCacheError"
variant: "Client"
tags: [rust, error, error/RedisCacheError]
---

# Error: RedisCacheError::Client

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs)

## Log Pattern / Error Message
`redis client error: {0}`

## Functions Generating/Handling this Error
- [[RedisCache_new|RedisCache::new]]
- [[RedisCache_get_connection|RedisCache::get_connection]]
- [[RedisCache_with_timeout|RedisCache::with_timeout]]
- [[BlockchainClient_new|BlockchainClient::new]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[RedisCache_new]].

### Remediation
- How to resolve the error in runtime or code.
