---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#2-standard
chunk_level: standard
chunk_type: table
heading: `AdmissionRequest`
token_count: 494
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
and