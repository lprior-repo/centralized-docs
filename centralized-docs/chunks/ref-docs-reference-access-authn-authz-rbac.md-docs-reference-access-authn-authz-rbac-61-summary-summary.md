---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#61-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 115
summary: #### Caution: Using wildcards in resource and verb entries could result in overly permissive access being granted to sensitive resources. For instance, if a new resource type is added, or a new...
---

#### Caution:
Using wildcards in resource and verb entries could result in overly permissive access being granted
to sensitive resources.
For instance, if a new resource type is added, or a new subresource is added,
or a new custom verb is checked, the wildcard entry automatically grants access, which may be undesirable.
The [principle of least privilege](/docs/concepts/security/rbac-good-practices/#least-privilege)
should be employed, using specific resources and verbs to ensure only the permissions required for the
workload to function correctly are applied.