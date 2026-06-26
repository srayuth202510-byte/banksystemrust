---
type: error_variant
enum: "BlockchainError"
variant: "DatabaseError"
tags: [rust, error, error/BlockchainError]
---

# Error: BlockchainError::DatabaseError

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs)

## Log Pattern / Error Message
`database error: {0}`

## Functions Generating/Handling this Error
- [[BlockchainClient_new|BlockchainClient::new]]
- [[BlockchainClient_get_transaction_status|BlockchainClient::get_transaction_status]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[BlockchainClient_new]].

### Remediation
- How to resolve the error in runtime or code.
