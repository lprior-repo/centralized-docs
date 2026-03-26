---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#8-standard
chunk_level: standard
chunk_type: prose
heading: Constrained Impersonation
token_count: 365
summary: #### Example: Impersonate a ServiceAccount To allow impersonating a service account named `app-sa` in the `production` namespace to create and update deployments: ``` `apiVersion:...
---

#### Example: Impersonate a ServiceAccount
To allow impersonating a service account named `app-sa` in the `production` namespace to create
and update deployments:
```
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
name: deputy-controller
namespace: default
`
```