---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#16-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 128
summary: | userInfo is information about the requesting user | |`object` [`k8s.io/apimachinery/pkg/runtime.RawExtension`](https://pkg.go.dev/k8s.io/apimachinery/pkg/runtime/#RawExtension)| object is the...
---

|
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