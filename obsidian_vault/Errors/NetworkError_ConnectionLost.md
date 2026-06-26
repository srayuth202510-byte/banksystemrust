---
type: error_variant
enum: "NetworkError"
variant: "ConnectionLost"
tags: [rust, error, error/NetworkError]
---

# Error: NetworkError::ConnectionLost

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs)

## Log Pattern / Error Message
`connection lost: {0}`

## Functions Generating/Handling this Error
- [[NetworkChannel_receive|NetworkChannel::receive]]
- [[NetworkChannel_send|NetworkChannel::send]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[NetworkChannel_receive]].

### Remediation
- How to resolve the error in runtime or code.
