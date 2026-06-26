---
type: error_variant
enum: "NetworkError"
variant: "BothFailed"
tags: [rust, error, error/NetworkError]
---

# Error: NetworkError::BothFailed

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs)

## Log Pattern / Error Message
`both protocols failed`

## Functions Generating/Handling this Error
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]
- [[NetworkChannel_connect|NetworkChannel::connect]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[P2pNode_send_kyc_inner]].

### Remediation
- How to resolve the error in runtime or code.
