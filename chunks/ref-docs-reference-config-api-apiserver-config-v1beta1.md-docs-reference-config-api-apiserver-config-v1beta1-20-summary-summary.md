---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#20-summary
chunk_level: summary
chunk_type: table
heading: `AnonymousAuthConfig`
token_count: 118
summary: ## `AnonymousAuthConfig` **Appears in:** * [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration) AnonymousAuthConfig provides the configuration for the anonymous...
---

## `AnonymousAuthConfig`
**Appears in:**
* [AuthenticationConfiguration](#apiserver-k8s-io-v1beta1-AuthenticationConfiguration)
AnonymousAuthConfig provides the configuration for the anonymous authenticator.
|Field|Description|
|`enabled`**[Required]**
`bool`|No description provided.|
|`conditions`**[Required]**
[`[]AnonymousAuthCondition`](#apiserver-k8s-io-v1beta1-AnonymousAuthCondition)|
If set, anonymous auth is only allowed if the request meets one of the
conditions.
|