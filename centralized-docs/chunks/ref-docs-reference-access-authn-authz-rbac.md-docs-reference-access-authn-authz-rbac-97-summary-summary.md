---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#97-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 117
summary: [\"Write Access for EndpointSlices\" section](#write-access-for-endpoints). | |**edit**|None|Allows read/write access to most objects in a namespace. This role does not allow viewing or modifying roles...
---

["Write Access for EndpointSlices" section](#write-access-for-endpoints).
|
|**edit**|None|Allows read/write access to most objects in a namespace.
This role does not allow viewing or modifying roles or role bindings.
However, this role allows accessing Secrets and running Pods as any ServiceAccount in
the namespace, so it can be used to gain the API access levels of any ServiceAccount in
the namespace. This role also does not allow write access to EndpointSlices in
clusters created using Kubernetes v1.22+. More information is available in the