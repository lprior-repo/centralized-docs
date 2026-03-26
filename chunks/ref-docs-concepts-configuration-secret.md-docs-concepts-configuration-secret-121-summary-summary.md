---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#121-summary
chunk_level: summary
chunk_type: prose
heading: Information security for Secrets
token_count: 123
summary: Authorization configuration affects how Secret data can be accessed within a namespace. For example, granting **list** or **watch** permissions on Secrets allows a subject to read all Secret data in...
---

Authorization configuration affects how Secret data can be accessed within a namespace.
For example, granting **list** or **watch** permissions on Secrets allows a subject
to read all Secret data in that namespace, not only the Secrets explicitly
referenced by its Pods. Restrict access to the minimum set of permissions
required for a workload to function, and avoid granting broad roles such as
`cluster-admin` unless required for administrative purposes.
Also see the [Authorization documentation](/docs/reference/access-authn-authz/rbac/).
A Secret is only sent to a node if a Pod on that node requires it.