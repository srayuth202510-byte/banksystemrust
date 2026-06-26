---
type: error_variant
enum: "CryptoError"
variant: "HsmError"
tags: [rust, error, error/CryptoError]
---

# Error: CryptoError::HsmError

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs)

## Log Pattern / Error Message
`hsm error: {0}`

## Functions Generating/Handling this Error
- [[HsmClient_new|HsmClient::new]]
- [[HsmClient_sign_ed25519|HsmClient::sign_ed25519]]
- [[HsmClient_find_key|HsmClient::find_key]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[HsmClient_new]].

### Remediation
- How to resolve the error in runtime or code.
