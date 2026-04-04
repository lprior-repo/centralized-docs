---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#124-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 123
summary: --- apiVersion: rbac.authorization.k8s.io/v1 kind: RoleBinding metadata: name: role-grantor-binding namespace: user-1-namespace roleRef: apiGroup: rbac.authorization.k8s.io kind: ClusterRole name:...
---

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