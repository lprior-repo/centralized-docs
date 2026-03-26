---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#38-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 126
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: impersonate-jane-identity rules: - apiGroups: [\"authentication.k8s.io\"] resources: [\"users\"] resourceNames:...
---

`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonate-jane-identity
rules:
- apiGroups: ["authentication.k8s.io"]
resources: ["users"]
resourceNames: ["jane.doe@example.com"]
verbs: ["impersonate:user-info"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: impersonate-jane-identity
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: impersonate-jane-identity