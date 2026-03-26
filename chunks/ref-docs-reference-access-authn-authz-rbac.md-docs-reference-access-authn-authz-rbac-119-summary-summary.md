---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#119-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 122
summary: * or explicitly allow specifying any permission in a `Role` or `ClusterRole` by giving them permission to perform the `escalate` verb on `roles` or `clusterroles` resources in the...
---

* or explicitly allow specifying any permission in a `Role` or `ClusterRole` by giving them
permission to perform the `escalate` verb on `roles` or `clusterroles` resources in the
`rbac.authorization.k8s.io` API group### Restrictions on role binding creation or update
You can only create/update a role binding if you already have all the permissions contained in the referenced role
(at the same scope as the role binding) *or* if you have been authorized to perform the `bind` verb on the referenced role.
For example, if `user-1`