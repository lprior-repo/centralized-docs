---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#47-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 104
summary: ``` `apiVersion: rbac.authorization.k8s.io/v1 # This cluster role binding allows anyone in the \"manager\" group to read secrets in any namespace. kind: ClusterRoleBinding metadata: name:...
---

```
`apiVersion: rbac.authorization.k8s.io/v1
# This cluster role binding allows anyone in the "manager" group to read secrets in any namespace.
kind: ClusterRoleBinding
metadata:
name: read-secrets-global
subjects:
- kind: Group
name: manager # Name is case sensitive
apiGroup: rbac.authorization.k8s.io
roleRef:
kind: ClusterRole
name: secret-reader
apiGroup: rbac.authorization.k8s.io
`
```