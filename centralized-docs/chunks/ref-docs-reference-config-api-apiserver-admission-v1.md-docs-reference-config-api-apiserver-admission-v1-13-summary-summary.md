---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#13-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 112
summary: (indicating the resource of the original API request). See documentation for the \"matchPolicy\" field in the webhook configuration type. | |`requestSubResource` `string`| requestSubResource is the...
---

 (indicating the resource of the original API request).
See documentation for the "matchPolicy" field in the webhook configuration type.
|
|`requestSubResource`
`string`|
requestSubResource is the name of the subresource of the original API request, if any (for example, "status" or "scale")
If this is specified and differs from the value in "subResource", an equivalent match and conversion was performed.
See documentation for the "matchPolicy" field in the webhook configuration type.
|
|`name`
`string`|