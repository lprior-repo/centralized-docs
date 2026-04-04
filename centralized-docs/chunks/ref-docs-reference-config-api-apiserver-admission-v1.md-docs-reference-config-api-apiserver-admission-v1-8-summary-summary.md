---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#8-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 106
summary: | subResource is the subresource being requested, if any (for example, \"status\" or \"scale\") | |`requestKind`...
---

|
subResource is the subresource being requested, if any (for example, "status" or "scale")
|
|`requestKind`
[`meta/v1.GroupVersionKind`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#GroupVersionKind)|
requestKind is the fully-qualified type of the original API request (for example, v1.Pod or autoscaling.v1.Scale).
If this is specified and differs from the value in "kind", an equivalent match and conversion was performed.