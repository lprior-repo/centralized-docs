---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#33-summary
chunk_level: summary
chunk_type: prose
heading: `PolicyRule`
token_count: 114
summary: | Resources that this rule matches. An empty list implies all kinds in all API groups. | |`namespaces` `[]string`| Namespaces that this rule matches. The empty string \"\" matches non-namespaced...
---

|
Resources that this rule matches. An empty list implies all kinds in all API groups.
|
|`namespaces`
`[]string`|
Namespaces that this rule matches.
The empty string "" matches non-namespaced resources.
An empty list implies every namespace.
|
|`nonResourceURLs`
`[]string`|
NonResourceURLs is a set of URL paths that should be audited.
`\*`s are allowed, but only as the full, final step in the path.
Examples:
* `/metrics` - Log requests for apiserver metrics