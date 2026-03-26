---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#47-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 125
summary: name: impersonate-app-sa namespace: default roleRef: apiGroup: rbac.authorization.k8s.io kind: Role name: impersonate-app-sa subjects: - kind: ServiceAccount name: deputy-controller namespace:...
---

name: impersonate-app-sa
namespace: default
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: Role
name: impersonate-app-sa
subjects:
- kind: ServiceAccount
name: deputy-controller
namespace: default
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
name: impersonate-manage-deployments
namespace: production
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: Role
name: impersonate-manage-deployments
subjects:
- kind: ServiceAccount