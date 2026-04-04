---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#54-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 66
summary: ``` `apiVersion: rbac.authorization.k8s.io/v1 kind: Role metadata: namespace: default name: pod-and-pod-logs-reader rules: - apiGroups: [\"\"] resources: [\"pods\", \"pods/log\"] verbs: [\"get\", \"list\"] `...
---

```
`apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
namespace: default
name: pod-and-pod-logs-reader
rules:
- apiGroups: [""]
resources: ["pods", "pods/log"]
verbs: ["get", "list"]
`
```