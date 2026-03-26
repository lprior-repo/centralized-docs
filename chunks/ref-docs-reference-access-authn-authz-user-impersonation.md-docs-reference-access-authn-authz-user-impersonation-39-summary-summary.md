---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#39-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 63
summary: roleRef: apiGroup: rbac.authorization.k8s.io kind: ClusterRole name: impersonate-jane-identity subjects: - kind: ServiceAccount name: my-controller namespace: default ` ``` **Step 2: Grant permission...
---

roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: impersonate-jane-identity
subjects:
- kind: ServiceAccount
name: my-controller
namespace: default
`
```
**Step 2: Grant permission to perform specific actions when impersonating**