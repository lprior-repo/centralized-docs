---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#29-standard
chunk_level: standard
chunk_type: prose
heading: Default roles and role bindings
token_count: 496
summary: ### Restrictions on role creation or update You can only create/update a role if at least one of the following things is true: 1. You already have all the permissions contained in the role, at the...
---

### Restrictions on role creation or update
You can only create/update a role if at least one of the following things is true:
1. You already have all the permissions contained in the role, at the same scope as the object being modified
(cluster-wide for a ClusterRole, within the same namespace or cluster-wide for a Role).
2. You are granted explicit permission to perform the `escalate` verb on the `roles` or
`clusterroles` resource in the `rbac.authorization.k8s.io` API group.
For example, if `user-1` does not have the ability to list Secrets cluster-wide, they cannot create a ClusterRole
containing that permission. To allow a user to create/update roles:
1. Grant them a role that allows them to create/update Role or ClusterRole objects, as desired.
2. Grant them permission to include specific permissions in the roles they create/update:
* implicitly, by giving them those permissions (if they attempt to create or modify a Role or
ClusterRole with permissions they themselves have not been granted, the API request will be forbidden)
* or explicitly allow specifying any permission in a `Role` or `ClusterRole` by giving them
permission to perform the `escalate` verb on `roles` or `clusterroles` resources in the
`rbac.authorization.k8s.io` API group### Restrictions on role binding creation or update
You can only create/update a role binding if you already have all the permissions contained in the referenced role
(at the same scope as the role binding) *or* if you have been authorized to perform the `bind` verb on the referenced role.
For example, if `user-1` does not have the ability to list Secrets cluster-wide, they cannot create a ClusterRoleBinding
to a role that grants that permission. To allow a user to create/update role bindings:
1. Grant them a role that allows them to create/update RoleBinding or ClusterRoleBinding objects, as desired.
2. Grant them permissions needed to bind a particular role:
* implicitly, by giving them the permissions contained in the role.
* explicitly, by giving them permission to perform the `bind` verb on the particular Role (or ClusterRole).
For example, this ClusterRole and RoleBinding would allow `user-1` to grant other users the `admin`, `edit`, and `view` roles in the namespace `user-1-namespace`: