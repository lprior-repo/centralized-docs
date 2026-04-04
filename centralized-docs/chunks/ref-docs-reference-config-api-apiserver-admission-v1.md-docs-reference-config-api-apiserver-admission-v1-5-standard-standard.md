---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#5-standard
chunk_level: standard
chunk_type: table
heading: `Operation`
token_count: 471
summary: ## `AdmissionResponse` **Appears in:** * [AdmissionReview](#admission-k8s-io-v1-AdmissionReview) AdmissionResponse describes an admission response. |Field|Description| |`uid`**[Required]**...
---

## `AdmissionResponse`
**Appears in:**
* [AdmissionReview](#admission-k8s-io-v1-AdmissionReview)
AdmissionResponse describes an admission response.
|Field|Description|
|`uid`**[Required]**
[`k8s.io/apimachinery/pkg/types.UID`](https://pkg.go.dev/k8s.io/apimachinery/pkg/types#UID)|
uid is an identifier for the individual request/response.
This must be copied over from the corresponding AdmissionRequest.
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
[`PatchType`](#admission-k8s-io-v1-PatchType)|
patchType is the type of Patch. Currently we only allow "JSONPatch".
|
|`auditAnnotations`
`map[string]string`|
auditAnnotations is an unstructured key value map set by remote admission controller (e.g. error=image-blacklisted).
MutatingAdmissionWebhook and ValidatingAdmissionWebhook admission controller will prefix the keys with
admission webhook name (e.g. imagepolicy.example.com/error=image-blacklisted). AuditAnnotations will be provided by
the admission webhook to add additional context to the audit log for this request.
|
|`warnings`
`[]string`|
warnings is a list of warning messages to return to the requesting API client.
Warning messages describe a problem the client making the API request should correct or be aware of.
Limit warnings to 120 characters if possible.
Warnings over 256 characters and large numbers of warnings may be truncated.
|
## `Operation`
(Alias of `string`)
**Appears in:**
* [AdmissionRequest](#admission-k8s-io-v1-AdmissionRequest)
Operation is the type of resource operation being checked for admission control