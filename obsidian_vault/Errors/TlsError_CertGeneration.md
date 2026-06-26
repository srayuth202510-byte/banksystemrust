---
type: error_variant
enum: "TlsError"
variant: "CertGeneration"
tags: [rust, error, error/TlsError]
---

# Error: TlsError::CertGeneration

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs)

## Log Pattern / Error Message
`certificate generation failed: {0}`

## Functions Generating/Handling this Error
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[TlsContext_generate_self_signed]].

### Remediation
- How to resolve the error in runtime or code.
