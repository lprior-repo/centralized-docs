---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#96-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 121
summary: |None|Allows admin access, intended to be granted within a namespace using a **RoleBinding**. If used in a **RoleBinding**, allows read/write access to most resources in a namespace, including the...
---

|None|Allows admin access, intended to be granted within a namespace using a **RoleBinding**.
If used in a **RoleBinding**, allows read/write access to most resources in a namespace,
including the ability to create roles and role bindings within the namespace.
This role does not allow write access to resource quota or to the namespace itself.
This role also does not allow write access to EndpointSlices in clusters created
using Kubernetes v1.22+. More information is available in the
["Write Access for EndpointSlices" section](#write-access-for-endpoints).
|
|**edit**