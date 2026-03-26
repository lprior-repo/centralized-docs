---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#52-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 123
summary: rules: - apiGroups: [\"\"] resources: [\"pods\"] verbs: - \"impersonate-on:arbitrary-node:list\" - \"impersonate-on:arbitrary-node:get\" --- apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRoleBinding...
---

rules:
- apiGroups: [""]
resources: ["pods"]
verbs:
- "impersonate-on:arbitrary-node:list"
- "impersonate-on:arbitrary-node:get"
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: impersonate-node-sa
namespace: default
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: impersonate-node-sa
subjects:
- kind: ServiceAccount
name: node-impersonator
namespace: default
---