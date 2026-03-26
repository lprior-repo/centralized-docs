---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#56-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 128
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: impersonate-associated-node-identity rules: - apiGroups: [\"authentication.k8s.io\"] resources: [\"nodes\"] verbs:...
---

`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonate-associated-node-identity
rules:
- apiGroups: ["authentication.k8s.io"]
resources: ["nodes"]
verbs: ["impersonate:associated-node"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonate-list-pods-on-node
rules:
- apiGroups: [""]
resources: ["pods"]
verbs:
- "impersonate-on:associated-node:list"
- "impersonate-on:associated-node:get"