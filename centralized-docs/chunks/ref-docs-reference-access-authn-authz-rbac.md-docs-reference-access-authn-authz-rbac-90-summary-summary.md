---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#90-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 128
summary: If you edit that ClusterRole, your changes will be overwritten on API server restart via [auto-reconciliation](#auto-reconciliation). To avoid that overwriting, either do not manually edit the role,...
---

If you edit that ClusterRole, your changes will be overwritten on API server restart
via [auto-reconciliation](#auto-reconciliation). To avoid that overwriting,
either do not manually edit the role, or disable auto-reconciliation.
Kubernetes RBAC API discovery roles|Default ClusterRole|Default ClusterRoleBinding|Description|
|**system:basic-user**|**system:authenticated** group|Allows a user read-only access to basic information about themselves. Prior to v1.14, this role was also bound to system:unauthenticated by default.|
|**system:discovery**|**system:authenticated**