---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#57-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 127
summary: resources: [\"pods\"] verbs: - \"impersonate-on:associated-node:list\" - \"impersonate-on:associated-node:get\" --- apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRoleBinding metadata: name:...
---

resources: ["pods"]
verbs:
- "impersonate-on:associated-node:list"
- "impersonate-on:associated-node:get"
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: node-agent-impersonate-node
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: impersonate-associated-node-identity
subjects:
- kind: ServiceAccount
name: node-agent
namespace: kube-system
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding