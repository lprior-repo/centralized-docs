---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#24-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 92
summary: #### Note: Impersonating a user or group allows you to perform any action as if you were that user or group; for that reason, impersonation is not namespace scoped. If you want to allow impersonation...
---

#### Note:
Impersonating a user or group allows you to perform any action as if you were that user or group;
for that reason, impersonation is not namespace scoped.
If you want to allow impersonation using Kubernetes RBAC,
this requires using a ClusterRole and a ClusterRoleBinding,
not a Role and RoleBinding.
Granting impersonation over ServiceAccounts is namespace scoped, but the impersonated ServiceAccount
could perform actions outside of namespace.