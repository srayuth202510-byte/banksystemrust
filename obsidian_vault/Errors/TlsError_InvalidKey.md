---
type: error_variant
enum: "TlsError"
variant: "InvalidKey"
tags: [rust, error, error/TlsError]
---

# Error: TlsError::InvalidKey

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs)

## Log Pattern / Error Message
`invalid key: {0}`

## Functions Generating/Handling this Error
- [[verify|verify]]
- [[sign|sign]]
- [[KeyPair_from_bytes|KeyPair::from_bytes]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[verify]].

### Remediation
- How to resolve the error in runtime or code.
