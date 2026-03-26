---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#30-standard
chunk_level: standard
chunk_type: prose
heading: Default roles and role bindings
token_count: 336
summary: * implicitly, by giving them the permissions contained in the role. * explicitly, by giving them permission to perform the `bind` verb on the particular Role (or ClusterRole). For example, this...
---

* implicitly, by giving them the permissions contained in the role.
* explicitly, by giving them permission to perform the `bind` verb on the particular Role (or ClusterRole).
For example, this ClusterRole and RoleBinding would allow `user-1` to grant other users the `admin`, `edit`, and `view` roles in the namespace `user-1-namespace`:
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: role-grantor
rules:
- apiGroups: ["rbac.authorization.k8s.io"]
resources: ["rolebindings"]
verbs: ["create"]
- apiGroups: ["rbac.authorization.k8s.io"]
resources: ["clusterroles"]
verbs: ["bind"]
# omit resourceNames to allow binding any ClusterRole
resourceNames: ["admin","edit","view"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
name: role-grantor-binding
namespace: user-1-namespace
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: role-grantor
subjects:
- apiGroup: rbac.authorization.k8s.io
kind: User
name: user-1
`
```
When bootstrapping the first roles and role bindings, it is necessary for the initial user to grant permissions they do not yet have.
To bootstrap initial roles and role bindings:
* Use a credential with the "system:masters" group, which is bound to the "cluster-admin" super-user role by the default bindings.## Command-line utilities