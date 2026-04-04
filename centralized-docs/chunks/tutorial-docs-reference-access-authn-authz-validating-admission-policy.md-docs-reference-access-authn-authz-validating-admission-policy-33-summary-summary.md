---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#33-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 43
summary: ``` `apiVersion: rules.example.com/v1 kind: ReplicaLimit metadata: name: \"replica-limit-test.example.com\" namespace: \"default\" maxReplicas: 3 ` ```
---

```
`apiVersion: rules.example.com/v1
kind: ReplicaLimit
metadata:
name: "replica-limit-test.example.com"
namespace: "default"
maxReplicas: 3
`
```