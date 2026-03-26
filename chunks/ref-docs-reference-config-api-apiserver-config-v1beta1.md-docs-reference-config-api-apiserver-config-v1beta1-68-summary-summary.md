---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#68-summary
chunk_level: summary
chunk_type: table
heading: `PrefixedClaimOrExpression`
token_count: 118
summary: * [ClaimMappings](#apiserver-k8s-io-v1beta1-ClaimMappings) PrefixedClaimOrExpression provides the configuration for a single prefixed claim or expression. |Field|Description| |`claim` `string`| claim...
---

* [ClaimMappings](#apiserver-k8s-io-v1beta1-ClaimMappings)
PrefixedClaimOrExpression provides the configuration for a single prefixed claim or expression.
|Field|Description|
|`claim`
`string`|
claim is the JWT claim to use.
Mutually exclusive with expression.
|
|`prefix`
`string`|
prefix is prepended to claim's value to prevent clashes with existing names.
prefix needs to be set if claim is set and can be the empty string.
Mutually exclusive with expression.
|
|`expression`
`string`|