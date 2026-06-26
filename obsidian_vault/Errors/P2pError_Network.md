---
type: error_variant
enum: "P2pError"
variant: "Network"
tags: [rust, error, error/P2pError]
---

# Error: P2pError::Network

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs)

## Log Pattern / Error Message
`network error: {0}`

## Functions Generating/Handling this Error
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]
- [[P2pNode_send_kyc|P2pNode::send_kyc]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[P2pNode_send_kyc_inner]].

### Remediation
- How to resolve the error in runtime or code.
