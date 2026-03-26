---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#7-standard
chunk_level: standard
chunk_type: prose
heading: Constrained Impersonation
token_count: 498
summary: ### Configuring constrained impersonation with RBAC All constrained impersonation permissions use the `authentication.k8s.io` API group. Here's how to configure the different modes. #### Example:...
---

### Configuring constrained impersonation with RBAC
All constrained impersonation permissions use the `authentication.k8s.io` API group. Here's how to
configure the different modes.
#### Example: Impersonate a user for specific actions
This example shows how to allow a service account to impersonate a user named `jane.doe@example.com`,
but only to `list` and `watch` pods in the `default` namespace. You need both a `ClusterRoleBinding`
for the identity permission and a `RoleBinding` for the action permission
**Step 1: Grant permission to impersonate the user identity**
```
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
subjects:
- kind: ServiceAccount
name: my-controller
namespace: default
`
```
**Step 2: Grant permission to perform specific actions when impersonating**
```
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
name: impersonate-list-watch-pods
subjects:
- kind: ServiceAccount
name: my-controller
namespace: default
`
```
Now the `my-controller` service account can impersonate `jane.doe@example.com` to list and watch
pods in the `default` namespace, but **cannot** perform other actions like deleting pods or
accessing resources in other namespaces.