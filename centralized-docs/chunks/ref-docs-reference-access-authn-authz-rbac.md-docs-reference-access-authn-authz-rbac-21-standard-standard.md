---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#21-standard
chunk_level: standard
chunk_type: code
heading: API objects
token_count: 392
summary: #### Note: * `system:serviceaccount:` (singular) is the prefix for service account usernames. * `system:serviceaccounts:` (plural) is the prefix for service account groups. #### RoleBinding examples...
---

#### Note:
* `system:serviceaccount:` (singular) is the prefix for service account usernames.
* `system:serviceaccounts:` (plural) is the prefix for service account groups.
#### RoleBinding examples
The following examples are `RoleBinding` excerpts that only
show the `subjects` section.
For a user named `alice@example.com`:
```
`subjects:
- kind: User
name: "alice@example.com"
apiGroup: rbac.authorization.k8s.io
`
```
For a group named `frontend-admins`:
```
`subjects:
- kind: Group
name: "frontend-admins"
apiGroup: rbac.authorization.k8s.io
`
```
For the default service account in the "kube-system" namespace:
```
`subjects:
- kind: ServiceAccount
name: default
namespace: kube-system
`
```
For all service accounts in the "qa" namespace:
```
`subjects:
- kind: Group
name: system:serviceaccounts:qa
apiGroup: rbac.authorization.k8s.io
`
```
For all service accounts in any namespace:
```
`subjects:
- kind: Group
name: system:serviceaccounts
apiGroup: rbac.authorization.k8s.io
`
```
For all authenticated users:
```
`subjects:
- kind: Group
name: system:authenticated
apiGroup: rbac.authorization.k8s.io
`
```
For all unauthenticated users:
```
`subjects:
- kind: Group
name: system:unauthenticated
apiGroup: rbac.authorization.k8s.io
`
```
For all users:
```
`subjects:
- kind: Group
name: system:authenticated
apiGroup: rbac.authorization.k8s.io
- kind: Group
name: system:unauthenticated
apiGroup: rbac.authorization.k8s.io
`
```