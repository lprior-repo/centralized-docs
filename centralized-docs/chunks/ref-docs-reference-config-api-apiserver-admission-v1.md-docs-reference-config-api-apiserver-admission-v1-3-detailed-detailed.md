---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#3-detailed
chunk_level: detailed
chunk_type: table
heading: `PatchType`
token_count: 1009
summary: \"status\" or \"scale\") If this is specified and differs from the value in \"subResource\", an equivalent match and conversion was performed. See documentation for the \"matchPolicy\" field in the webhook...
---

"status" or "scale")
If this is specified and differs from the value in "subResource", an equivalent match and conversion was performed.
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
operation is the operation being performed. This may be different than the operation
requested. e.g. a patch can result in either a CREATE or UPDATE Operation.
|
|`userInfo`**[Required]**
[`authentication/v1.UserInfo`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#userinfo-v1-authentication-k8s-io)|
userInfo is information about the requesting user
|
|`object`
[`k8s.io/apimachinery/pkg/runtime.RawExtension`](https://pkg.go.dev/k8s.io/apimachinery/pkg/runtime/#RawExtension)|
object is the object from the incoming request.
|
|`oldObject`
[`k8s.io/apimachinery/pkg/runtime.RawExtension`](https://pkg.go.dev/k8s.io/apimachinery/pkg/runtime/#RawExtension)|
oldObject is the existing object. Only populated for DELETE and UPDATE requests.
|
|`dryRun`
`bool`|
dryRun indicates that modifications will definitely not be persisted for this request.
Defaults to false.
|
|`options`
[`k8s.io/apimachinery/pkg/runtime.RawExtension`](https://pkg.go.dev/k8s.io/apimachinery/pkg/runtime/#RawExtension)|
options is the operation option structure of the operation being performed.
e.g. `meta.k8s.io/v1.DeleteOptions` or `meta.k8s.io/v1.CreateOptions`. This may be
different than the options the caller provided. e.g. for a patch request the performed
Operation might be a CREATE, in which case the Options will a
`meta.k8s.io/v1.CreateOptions` even though the caller provided `meta.k8s.io/v1.PatchOptions`.
|
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
## `PatchType`
(Alias of `string`)
**Appears in:**
* [AdmissionResponse](#admission-k8s-io-v1-AdmissionResponse)
PatchType is the type of patch being used to represent the mutated object