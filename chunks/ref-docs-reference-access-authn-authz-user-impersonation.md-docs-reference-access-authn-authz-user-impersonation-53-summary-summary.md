---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#53-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 107
summary: name: impersonate-node-sa subjects: - kind: ServiceAccount name: node-impersonator namespace: default --- apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRoleBinding metadata: name:...
---

name: impersonate-node-sa
subjects:
- kind: ServiceAccount
name: node-impersonator
namespace: default
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: impersonate-list-pods
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: impersonate-list-pods
subjects:
- kind: ServiceAccount
name: node-impersonator
namespace: default
`
```