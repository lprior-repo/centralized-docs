---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#40-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 81
summary: `params` will be null if `paramKind` of the policy, or `paramRef` of the binding are not specified. For the use cases requiring parameter configuration, we recommend to add a param check in...
---

`params` will be null
if `paramKind` of the policy, or `paramRef` of the binding are not specified.
For the use cases requiring parameter configuration, we recommend to add a param check in
`spec.validations[0].expression`:
```
`- expression: "params != null"
message: "params missing but required to bind to this policy"
`
```