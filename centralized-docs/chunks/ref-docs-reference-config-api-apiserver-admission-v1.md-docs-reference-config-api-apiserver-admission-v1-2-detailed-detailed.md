---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#2-detailed
chunk_level: detailed
chunk_type: table
heading: `AdmissionRequest`
token_count: 995
summary: * [AdmissionReview](#admission-k8s-io-v1-AdmissionReview) AdmissionRequest describes the admission.Attributes for the admission request. |Field|Description| |`uid`**[Required]**...
---

* [AdmissionReview](#admission-k8s-io-v1-AdmissionReview)
AdmissionRequest describes the admission.Attributes for the admission request.
|Field|Description|
|`uid`**[Required]**
[`k8s.io/apimachinery/pkg/types.UID`](https://pkg.go.dev/k8s.io/apimachinery/pkg/types#UID)|
uid is an identifier for the individual request/response. It allows us to distinguish instances of requests which are
otherwise identical (parallel requests, requests when earlier requests did not modify etc)
The UID is meant to track the round trip (request/response) between the KAS and the WebHook, not the user request.
It is suitable for correlating log entries between the webhook and apiserver, for either auditing or debugging.
|
|`kind`**[Required]**
[`meta/v1.GroupVersionKind`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#GroupVersionKind)|
kind is the fully-qualified type of object being submitted (for example, v1.Pod or autoscaling.v1.Scale)
|
|`resource`**[Required]**
[`meta/v1.GroupVersionResource`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#GroupVersionResource)|
resource is the fully-qualified resource being requested (for example, v1.pods)
|
|`subResource`
`string`|
subResource is the subresource being requested, if any (for example, "status" or "scale")
|
|`requestKind`
[`meta/v1.GroupVersionKind`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#GroupVersionKind)|
requestKind is the fully-qualified type of the original API request (for example, v1.Pod or autoscaling.v1.Scale).
If this is specified and differs from the value in "kind", an equivalent match and conversion was performed.
For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook registered a rule of
`apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]` and `matchPolicy: Equivalent`,
an API request to apps/v1beta1 deployments would be converted and sent to the webhook
with `kind: {group:"apps", version:"v1", kind:"Deployment"}` (matching the rule the webhook registered for),
and `requestKind: {group:"apps", version:"v1beta1", kind:"Deployment"}` (indicating the kind of the original API request).
See documentation for the "matchPolicy" field in the webhook configuration type for more details.
|
|`requestResource`
[`meta/v1.GroupVersionResource`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#GroupVersionResource)|
requestResource is the fully-qualified resource of the original API request (for example, v1.pods).
If this is specified and differs from the value in "resource", an equivalent match and conversion was performed.
For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook registered a rule of
`apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]` and `matchPolicy: Equivalent`,
an API request to apps/v1beta1 deployments would be converted and sent to the webhook
with `resource: {group:"apps", version:"v1", resource:"deployments"}` (matching the resource the webhook registered for),
and `requestResource: {group:"apps", version:"v1beta1", resource:"deployments"}` (indicating the resource of the original API request).
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