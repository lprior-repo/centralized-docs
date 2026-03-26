---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#116-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 123
summary: 1. You already have all the permissions contained in the role, at the same scope as the object being modified (cluster-wide for a ClusterRole, within the same namespace or cluster-wide for a Role)....
---

1. You already have all the permissions contained in the role, at the same scope as the object being modified
(cluster-wide for a ClusterRole, within the same namespace or cluster-wide for a Role).
2. You are granted explicit permission to perform the `escalate` verb on the `roles` or
`clusterroles` resource in the `rbac.authorization.k8s.io` API group.
For example, if `user-1` does not have the ability to list Secrets cluster-wide, they cannot create a ClusterRole
containing that permission. To allow a user to create/update roles: