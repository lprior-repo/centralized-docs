---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#41-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 122
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: Role metadata: name: impersonate-list-watch-pods namespace: default rules: - apiGroups: [\"\"] resources: [\"pods\"] verbs: -...
---

`apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
name: impersonate-list-watch-pods
namespace: default
rules:
- apiGroups: [""]
resources: ["pods"]
verbs:
- "impersonate-on:user-info:list"
- "impersonate-on:user-info:watch"
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
name: impersonate-list-watch-pods
namespace: default
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: Role