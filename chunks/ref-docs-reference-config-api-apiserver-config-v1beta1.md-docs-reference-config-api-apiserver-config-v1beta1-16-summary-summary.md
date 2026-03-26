---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#16-summary
chunk_level: summary
chunk_type: table
heading: `AuthorizationConfiguration`
token_count: 111
summary: ## `AuthorizationConfiguration` |Field|Description| |`apiVersion` string|`apiserver.k8s.io/v1beta1`| |`kind` string|`AuthorizationConfiguration`| |`authorizers`**[Required]**...
---

## `AuthorizationConfiguration`
|Field|Description|
|`apiVersion`
string|`apiserver.k8s.io/v1beta1`|
|`kind`
string|`AuthorizationConfiguration`|
|`authorizers`**[Required]**
[`[]AuthorizerConfiguration`](#apiserver-k8s-io-v1beta1-AuthorizerConfiguration)|
Authorizers is an ordered list of authorizers to
authorize requests against.
This is similar to the --authorization-modes kube-apiserver flag
Must be at least one.
|