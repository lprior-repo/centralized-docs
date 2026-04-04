---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#4-standard
chunk_level: standard
chunk_type: prose
heading: `AdmissionRequest`
token_count: 457
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