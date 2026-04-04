---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#21-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionResponse`
token_count: 125
summary: | |`allowed`**[Required]** `bool`| allowed indicates whether or not the admission request was permitted. | |`status`...
---

|
|`allowed`**[Required]**
`bool`|
allowed indicates whether or not the admission request was permitted.
|
|`status`
[`meta/v1.Status`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#status-v1-meta)|
status is the result contains extra details into why an admission request was denied.
This field IS NOT consulted in any way if "Allowed" is "true".
|
|`patch`
`[]byte`|
patch is the patch body. Currently we only support "JSONPatch" which implements RFC 6902.
|
|`patchType`