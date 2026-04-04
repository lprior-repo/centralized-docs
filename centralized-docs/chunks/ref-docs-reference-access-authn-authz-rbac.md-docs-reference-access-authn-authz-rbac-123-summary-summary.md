---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#123-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 125
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: name: role-grantor rules: - apiGroups: [\"rbac.authorization.k8s.io\"] resources: [\"rolebindings\"] verbs: [\"create\"] - apiGroups:...
---

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