---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#85-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 109
summary: ## Default roles and role bindings API servers create a set of default ClusterRole and ClusterRoleBinding objects. Many of these are `system:` prefixed, which indicates that the resource is directly...
---

## Default roles and role bindings
API servers create a set of default ClusterRole and ClusterRoleBinding objects.
Many of these are `system:` prefixed, which indicates that the resource is directly
managed by the cluster control plane.
All of the default ClusterRoles and ClusterRoleBindings are labeled with `kubernetes.io/bootstrapping=rbac-defaults`.
#### Caution:
Take care when modifying ClusterRoles and ClusterRoleBindings with names
that have a `system:` prefix.
Modifications to these resources can result in non-functional clusters.