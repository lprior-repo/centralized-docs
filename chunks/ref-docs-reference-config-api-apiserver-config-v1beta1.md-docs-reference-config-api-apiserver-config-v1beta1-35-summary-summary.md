---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#35-summary
chunk_level: summary
chunk_type: table
heading: `ClaimOrExpression`
token_count: 105
summary: * [ClaimMappings](#apiserver-k8s-io-v1beta1-ClaimMappings) ClaimOrExpression provides the configuration for a single claim or expression. |Field|Description| |`claim` `string`| claim is the JWT claim...
---

* [ClaimMappings](#apiserver-k8s-io-v1beta1-ClaimMappings)
ClaimOrExpression provides the configuration for a single claim or expression.
|Field|Description|
|`claim`
`string`|
claim is the JWT claim to use.
Either claim or expression must be set.
Mutually exclusive with expression.
|
|`expression`
`string`|
expression represents the expression which will be evaluated by CEL.
CEL expressions have access to the contents of the token claims, organized into CEL variable: