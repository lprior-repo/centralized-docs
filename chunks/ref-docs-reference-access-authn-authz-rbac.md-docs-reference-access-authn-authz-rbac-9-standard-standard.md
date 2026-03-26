---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#9-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 174
summary: ### RoleBinding and ClusterRoleBinding A role binding grants the permissions defined in a role to a user or set of users. It holds a list of *subjects* (users, groups, or service accounts), and a...
---

### RoleBinding and ClusterRoleBinding
A role binding grants the permissions defined in a role to a user or set of users.
It holds a list of *subjects* (users, groups, or service accounts), and a reference to the
role being granted.
A RoleBinding grants permissions within a specific namespace whereas a ClusterRoleBinding
grants that access cluster-wide.
A RoleBinding may reference any Role in the same namespace. Alternatively, a RoleBinding
can reference a ClusterRole and bind that ClusterRole to the namespace of the RoleBinding.
If you want to bind a ClusterRole to all the namespaces in your cluster, you use a
ClusterRoleBinding.
The name of a RoleBinding or ClusterRoleBinding object must be a valid
[path segment name](/docs/concepts/overview/working-with-objects/names/#path-segment-names).