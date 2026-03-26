---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#58-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 106
summary: name: node-agent namespace: kube-system --- apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRoleBinding metadata: name: node-agent-impersonate-list-pods roleRef: apiGroup:...
---

name: node-agent
namespace: kube-system
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: node-agent-impersonate-list-pods
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: impersonate-list-pods-on-node
subjects:
- kind: ServiceAccount
name: node-agent
namespace: kube-system
`
```
The controller would get the node name using the downward API: