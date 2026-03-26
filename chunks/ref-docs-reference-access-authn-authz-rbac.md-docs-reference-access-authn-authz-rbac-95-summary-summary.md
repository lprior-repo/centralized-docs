---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#95-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 122
summary: |Default ClusterRole|Default ClusterRoleBinding|Description| |**cluster-admin**|**system:masters** group|Allows super-user access to perform any action on any resource. When used in a...
---

|Default ClusterRole|Default ClusterRoleBinding|Description|
|**cluster-admin**|**system:masters** group|Allows super-user access to perform any action on any resource.
When used in a **ClusterRoleBinding**, it gives full control over every resource in the cluster and in all namespaces.
When used in a **RoleBinding**, it gives full control over every resource in the role binding's namespace, including the namespace itself.|
|**admin**|None|Allows admin access, intended to be granted within a namespace using a **RoleBinding**.
If used in a **RoleBinding**