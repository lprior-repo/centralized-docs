---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#10-detailed
chunk_level: detailed
chunk_type: code
heading: API objects
token_count: 659
summary: ### Referring to subjects A RoleBinding or ClusterRoleBinding binds a role to subjects. Subjects can be groups, users or...
---

### Referring to subjects
A RoleBinding or ClusterRoleBinding binds a role to subjects.
Subjects can be groups, users or
[ServiceAccounts](/docs/tasks/configure-pod-container/configure-service-account/).
Kubernetes represents usernames as strings.
These can be: plain names, such as "alice"; email-style names, like "bob@example.com";
or numeric user IDs represented as a string. It is up to you as a cluster administrator
to configure the [authentication modules](/docs/reference/access-authn-authz/authentication/)
so that authentication produces usernames in the format you want.
#### Caution:
The prefix `system:` is reserved for Kubernetes system use, so you should ensure
that you don't have users or groups with names that start with `system:` by
accident.
Other than this special prefix, the RBAC authorization system does not require any format
for usernames.
In Kubernetes, Authenticator modules provide group information.
Groups, like users, are represented as strings, and that string has no format requirements,
other than that the prefix `system:` is reserved.
[ServiceAccounts](/docs/tasks/configure-pod-container/configure-service-account/) have names prefixed
with `system:serviceaccount:`, and belong to groups that have names prefixed with `system:serviceaccounts:`.
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