---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#48-summary
chunk_level: summary
chunk_type: table
heading: `ExtraMapping`
token_count: 128
summary: * [ClaimMappings](#apiserver-k8s-io-v1beta1-ClaimMappings) ExtraMapping provides the configuration for a single extra mapping. |Field|Description| |`key`**[Required]** `string`| key is a string to...
---

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