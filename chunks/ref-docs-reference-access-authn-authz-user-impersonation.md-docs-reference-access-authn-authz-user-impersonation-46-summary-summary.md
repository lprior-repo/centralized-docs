---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#46-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 121
summary: kind: Role metadata: name: impersonate-manage-deployments namespace: production rules: - apiGroups: [\"apps\"] resources: [\"deployments\"] verbs: - \"impersonate-on:serviceaccount:create\" -...
---

kind: Role
metadata:
name: impersonate-manage-deployments
namespace: production
rules:
- apiGroups: ["apps"]
resources: ["deployments"]
verbs:
- "impersonate-on:serviceaccount:create"
- "impersonate-on:serviceaccount:update"
- "impersonate-on:serviceaccount:patch"
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
name: impersonate-app-sa
namespace: default
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: Role