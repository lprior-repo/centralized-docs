---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#25-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 114
summary: and ClusterRole) because a Kubernetes object always has to be either namespaced or not namespaced; it can't be both. ClusterRoles have several uses. You can use a ClusterRole to: 1. define...
---

and ClusterRole) because a Kubernetes object always has to be either namespaced or not namespaced;
it can't be both.
ClusterRoles have several uses. You can use a ClusterRole to:
1. define permissions on namespaced resources and be granted access within individual namespace(s)
2. define permissions on namespaced resources and be granted access across all namespaces
3. define permissions on cluster-scoped resources
If you want to define a role within a namespace, use a Role; if you want to define
a role cluster-wide, use a ClusterRole.