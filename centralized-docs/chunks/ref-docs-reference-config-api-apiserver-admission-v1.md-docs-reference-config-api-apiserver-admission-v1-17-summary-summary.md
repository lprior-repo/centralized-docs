---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#17-summary
chunk_level: summary
chunk_type: prose
heading: `AdmissionRequest`
token_count: 120
summary: | |`dryRun` `bool`| dryRun indicates that modifications will definitely not be persisted for this request. Defaults to false. | |`options`...
---

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