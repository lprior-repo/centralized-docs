---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#80-summary
chunk_level: summary
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 112
summary: expression: 'request.resource.group != \"rbac.authorization.k8s.io\"' validations: - expression: \"!object.metadata.name.contains('demo') || object.metadata.namespace == 'demo'\" ` ``` Match conditions...
---

expression: 'request.resource.group != "rbac.authorization.k8s.io"'
validations:
- expression: "!object.metadata.name.contains('demo') || object.metadata.namespace == 'demo'"
`
```
Match conditions have access to the same CEL variables as validation expressions.
In the event of an error evaluating a match condition the policy is not evaluated. Whether to reject
the request is determined as follows:
1. If **any** match condition evaluated to `false` (regardless of other errors), the API server skips the policy.
2. Otherwise: