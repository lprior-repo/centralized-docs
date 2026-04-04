---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#20-summary
chunk_level: summary
chunk_type: table
heading: `AdmissionResponse`
token_count: 120
summary: * [AdmissionReview](#admission-k8s-io-v1-AdmissionReview) AdmissionResponse describes an admission response. |Field|Description| |`uid`**[Required]**...
---

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