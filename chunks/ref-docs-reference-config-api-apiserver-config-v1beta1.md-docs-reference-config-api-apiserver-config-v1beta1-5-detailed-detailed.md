---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#5-detailed
chunk_level: detailed
chunk_type: table
heading: `ExtraMapping`
token_count: 344
summary: ## `EgressSelectorType` (Alias of `string`) **Appears in:** * [Issuer](#apiserver-k8s-io-v1beta1-Issuer) EgressSelectorType is an indicator of which egress selection should be used for sending...
---

## `EgressSelectorType`
(Alias of `string`)
**Appears in:**
* [Issuer](#apiserver-k8s-io-v1beta1-Issuer)
EgressSelectorType is an indicator of which egress selection should be used for sending traffic.
## `ExtraMapping`
**Appears in:**
* [ClaimMappings](#apiserver-k8s-io-v1beta1-ClaimMappings)
ExtraMapping provides the configuration for a single extra mapping.
|Field|Description|
|`key`**[Required]**
`string`|
key is a string to use as the extra attribute key.
key must be a domain-prefix path (e.g. example.org/foo). All characters before the first "/" must be a valid
subdomain as defined by RFC 1123. All characters trailing the first "/" must
be valid HTTP Path characters as defined by RFC 3986.
key must be lowercase.
Required to be unique.
|
|`valueExpression`**[Required]**
`string`|
valueExpression is a CEL expression to extract extra attribute value.
valueExpression must produce a string or string array value.
"", [], and null values are treated as the extra mapping not being present.
Empty string values contained within a string array are filtered out.
CEL expressions have access to the contents of the token claims, organized into CEL variable:
* 'claims' is a map of claim names to claim values.
For example, a variable named 'sub' can be accessed as 'claims.sub'.
Nested claims can be accessed using dot notation, e.g. 'claims.foo.bar'.
Documentation on CEL: https://kubernetes.io/docs/reference/using-api/cel/
|