---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#7-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 121
summary: | kind is the fully-qualified type of object being submitted (for example, v1.Pod or autoscaling.v1.Scale) | |`resource`**[Required]**...
---

|
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