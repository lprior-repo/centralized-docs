---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#21-summary
chunk_level: summary
chunk_type: table
heading: `AuthenticationMetadata`
token_count: 84
summary: ## `AuthenticationMetadata` **Appears in:** * [Event](#audit-k8s-io-v1-Event)|Field|Description| |`impersonationConstraint` `string`| ImpersonationConstraint is the verb associated with the...
---

## `AuthenticationMetadata`
**Appears in:**
* [Event](#audit-k8s-io-v1-Event)|Field|Description|
|`impersonationConstraint`
`string`|
ImpersonationConstraint is the verb associated with the constrained impersonation mode that was used to authorize
the ImpersonatedUser associated with this audit event. It is only set when constrained impersonation was used.
|