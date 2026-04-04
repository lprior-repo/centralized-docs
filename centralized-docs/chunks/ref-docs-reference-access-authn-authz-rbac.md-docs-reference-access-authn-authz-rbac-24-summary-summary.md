---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#24-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 122
summary: An RBAC *Role* or *ClusterRole* contains rules that represent a set of permissions. Permissions are purely additive (there are no \"deny\" rules). A Role always sets permissions within a particular...
---

An RBAC *Role* or *ClusterRole* contains rules that represent a set of permissions.
Permissions are purely additive (there are no "deny" rules).
A Role always sets permissions within a particular [namespace](/docs/concepts/overview/working-with-objects/namespaces);
when you create a Role, you have to specify the namespace it belongs in.
ClusterRole, by contrast, is a non-namespaced resource. The resources have different names (Role
and ClusterRole) because a Kubernetes object always has to be either namespaced or not namespaced;
it can't be both.