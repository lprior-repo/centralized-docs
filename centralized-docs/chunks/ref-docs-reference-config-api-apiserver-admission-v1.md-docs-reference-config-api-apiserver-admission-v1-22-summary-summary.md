---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#22-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionResponse`
token_count: 117
summary: | patch is the patch body. Currently we only support \"JSONPatch\" which implements RFC 6902. | |`patchType` [`PatchType`](#admission-k8s-io-v1-PatchType)| patchType is the type of Patch. Currently we...
---

|
patch is the patch body. Currently we only support "JSONPatch" which implements RFC 6902.
|
|`patchType`
[`PatchType`](#admission-k8s-io-v1-PatchType)|
patchType is the type of Patch. Currently we only allow "JSONPatch".
|
|`auditAnnotations`
`map[string]string`|
auditAnnotations is an unstructured key value map set by remote admission controller (e.g. error=image-blacklisted).
MutatingAdmissionWebhook and ValidatingAdmissionWebhook admission controller will prefix the keys with