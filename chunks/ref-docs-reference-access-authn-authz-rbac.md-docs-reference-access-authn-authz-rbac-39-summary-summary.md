---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#39-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 123
summary: `apiVersion: rbac.authorization.k8s.io/v1 # This role binding allows \"jane\" to read pods in the \"default\" namespace. # You need to already have a Role named \"pod-reader\" in that namespace. kind:...
---

`apiVersion: rbac.authorization.k8s.io/v1
# This role binding allows "jane" to read pods in the "default" namespace.
# You need to already have a Role named "pod-reader" in that namespace.
kind: RoleBinding
metadata:
name: read-pods
namespace: default
subjects:
# You can specify more than one "subject"
- kind: User
name: jane # "name" is case sensitive
apiGroup: rbac.authorization.k8s.io
roleRef:
# "roleRef" specifies the binding to a Role / ClusterRole