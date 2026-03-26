---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#51-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 124
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: impersonate-node-sa rules: - apiGroups: [\"authentication.k8s.io\"] resources: [\"nodes\"] resourceNames: [\"mynode\"] verbs:...
---

`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonate-node-sa
rules:
- apiGroups: ["authentication.k8s.io"]
resources: ["nodes"]
resourceNames: ["mynode"]
verbs: ["impersonate:arbitrary-node"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonate-list-pods
rules:
- apiGroups: [""]
resources: ["pods"]
verbs:
- "impersonate-on:arbitrary-node:list"