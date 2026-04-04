---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#14-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 119
summary: See documentation for the \"matchPolicy\" field in the webhook configuration type. | |`name` `string`| name is the name of the object as presented in the request. On a CREATE operation, the client may...
---

See documentation for the "matchPolicy" field in the webhook configuration type.
|
|`name`
`string`|
name is the name of the object as presented in the request. On a CREATE operation, the client may omit name and
rely on the server to generate the name. If that is the case, this field will contain an empty string.
|
|`namespace`
`string`|
namespace is the namespace associated with the request (if any).
|
|`operation`**[Required]**
[`Operation`](#admission-k8s-io-v1-Operation)|