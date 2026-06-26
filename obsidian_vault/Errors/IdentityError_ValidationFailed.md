---
type: error_variant
enum: "IdentityError"
variant: "ValidationFailed"
tags: [rust, error, error/IdentityError]
---

# Error: IdentityError::ValidationFailed

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs)

## Log Pattern / Error Message
`validation failed: {0}`

## Functions Generating/Handling this Error
- [[KycData_compute_hash|KycData::compute_hash]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[KycData_compute_hash]].

### Remediation
- How to resolve the error in runtime or code.
