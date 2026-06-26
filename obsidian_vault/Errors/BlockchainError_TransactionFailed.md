---
type: error_variant
enum: "BlockchainError"
variant: "TransactionFailed"
tags: [rust, error, error/BlockchainError]
---

# Error: BlockchainError::TransactionFailed

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs)

## Log Pattern / Error Message
`transaction failed: {0}`

## Functions Generating/Handling this Error
- [[BlockchainClient_send_to_node|BlockchainClient::send_to_node]]
- [[BlockchainClient_submit|BlockchainClient::submit]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[BlockchainClient_send_to_node]].

### Remediation
- How to resolve the error in runtime or code.
