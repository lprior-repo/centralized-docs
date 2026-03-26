---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#8-standard
chunk_level: standard
chunk_type: table
heading: `ClaimOrExpression`
token_count: 189
summary: ## `ClaimOrExpression` **Appears in:** * [ClaimMappings](#apiserver-k8s-io-v1beta1-ClaimMappings) ClaimOrExpression provides the configuration for a single claim or expression. |Field|Description|...
---

## `ClaimOrExpression`
**Appears in:**
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
* 'claims' is a map of claim names to claim values.
For example, a variable named 'sub' can be accessed as 'claims.sub'.
Nested claims can be accessed using dot notation, e.g. 'claims.foo.bar'.
Documentation on CEL: https://kubernetes.io/docs/reference/using-api/cel/
Mutually exclusive with claim.
|