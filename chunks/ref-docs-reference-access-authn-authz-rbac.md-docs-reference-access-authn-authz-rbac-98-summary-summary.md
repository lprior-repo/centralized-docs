---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#98-summary
chunk_level: summary
chunk_type: table
heading: Default roles and role bindings
token_count: 110
summary: clusters created using Kubernetes v1.22+. More information is available in the [\"Write Access for EndpointSlices\" section](#write-access-for-endpoints). | |**view**|None|Allows read-only access to...
---

clusters created using Kubernetes v1.22+. More information is available in the
["Write Access for EndpointSlices" section](#write-access-for-endpoints).
|
|**view**|None|Allows read-only access to see most objects in a namespace.
It does not allow viewing roles or role bindings.
This role does not allow viewing Secrets, since reading
the contents of Secrets enables access to ServiceAccount credentials
in the namespace, which would allow API access as any ServiceAccount
in the namespace (a form of privilege escalation).
|