---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#11-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 146
summary: ``` `apiVersion: rbac.authorization.k8s.io/v1 # This role binding allows \"dave\" to read secrets in the \"development\" namespace. # You need to already have a ClusterRole named \"secret-reader\". kind:...
---

```
`apiVersion: rbac.authorization.k8s.io/v1
# This role binding allows "dave" to read secrets in the "development" namespace.
# You need to already have a ClusterRole named "secret-reader".
kind: RoleBinding
metadata:
name: read-secrets
# The namespace of the RoleBinding determines where the permissions are granted.
# This only grants permissions within the "development" namespace.
namespace: development
subjects:
- kind: User
name: dave # Name is case sensitive
apiGroup: rbac.authorization.k8s.io
roleRef:
kind: ClusterRole
name: secret-reader
apiGroup: rbac.authorization.k8s.io
`
```