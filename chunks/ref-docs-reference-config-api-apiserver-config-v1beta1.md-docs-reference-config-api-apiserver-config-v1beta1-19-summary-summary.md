---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#19-summary
chunk_level: summary
chunk_type: table
heading: `AnonymousAuthCondition`
token_count: 76
summary: ## `AnonymousAuthCondition` **Appears in:** * [AnonymousAuthConfig](#apiserver-k8s-io-v1beta1-AnonymousAuthConfig) AnonymousAuthCondition describes the condition under which anonymous auth should be...
---

## `AnonymousAuthCondition`
**Appears in:**
* [AnonymousAuthConfig](#apiserver-k8s-io-v1beta1-AnonymousAuthConfig)
AnonymousAuthCondition describes the condition under which anonymous auth
should be enabled.
|Field|Description|
|`path`**[Required]**
`string`|
Path for which anonymous auth is enabled.
|