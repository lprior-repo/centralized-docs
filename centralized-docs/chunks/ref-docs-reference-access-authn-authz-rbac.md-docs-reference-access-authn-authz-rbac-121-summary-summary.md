---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#121-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 78
summary: * implicitly, by giving them the permissions contained in the role. * explicitly, by giving them permission to perform the `bind` verb on the particular Role (or ClusterRole). For example, this...
---

* implicitly, by giving them the permissions contained in the role.
* explicitly, by giving them permission to perform the `bind` verb on the particular Role (or ClusterRole).
For example, this ClusterRole and RoleBinding would allow `user-1` to grant other users the `admin`, `edit`, and `view` roles in the namespace `user-1-namespace`: