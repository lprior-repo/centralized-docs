---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#49-summary
chunk_level: summary
chunk_type: prose
heading: `ExtraMapping`
token_count: 109
summary: \"/\" must be valid HTTP Path characters as defined by RFC 3986. key must be lowercase. Required to be unique. | |`valueExpression`**[Required]** `string`| valueExpression is a CEL expression to...
---

"/" must
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