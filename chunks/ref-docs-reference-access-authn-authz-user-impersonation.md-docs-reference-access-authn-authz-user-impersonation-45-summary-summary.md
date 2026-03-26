---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#45-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 127
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: Role metadata: name: impersonate-app-sa namespace: default rules: - apiGroups: [\"authentication.k8s.io\"] resources: [\"serviceaccounts\"] resourceNames:...
---

`apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
name: impersonate-app-sa
namespace: default
rules:
- apiGroups: ["authentication.k8s.io"]
resources: ["serviceaccounts"]
resourceNames: ["app-sa"]
# For service accounts, you must specify the namespace in the RoleBinding
verbs: ["impersonate:serviceaccount"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
name: impersonate-manage-deployments
namespace: production
rules:
- apiGroups: ["apps"]