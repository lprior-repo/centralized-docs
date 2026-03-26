---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#18-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 58
summary: ``` `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: impersonator rules: - apiGroups: [\"\"] resources: [\"users\", \"groups\", \"serviceaccounts\"] verbs: [\"impersonate\"] ` ```
---

```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonator
rules:
- apiGroups: [""]
resources: ["users", "groups", "serviceaccounts"]
verbs: ["impersonate"]
`
```