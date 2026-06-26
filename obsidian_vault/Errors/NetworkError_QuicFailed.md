---
type: error_variant
enum: "NetworkError"
variant: "QuicFailed"
tags: [rust, error, error/NetworkError]
---

# Error: NetworkError::QuicFailed

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs)

## Log Pattern / Error Message
`quic connection failed: {0}`

## Functions Generating/Handling this Error
- [[connect_quic|connect::quic]]
- [[start_quic_server|start::quic_server]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[connect_quic]].

### Remediation
- How to resolve the error in runtime or code.
