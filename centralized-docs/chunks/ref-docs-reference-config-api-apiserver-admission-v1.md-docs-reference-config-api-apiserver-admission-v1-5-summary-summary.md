---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#5-summary
chunk_level: summary
chunk_type: table
heading: `AdmissionRequest`
token_count: 109
summary: * [AdmissionReview](#admission-k8s-io-v1-AdmissionReview) AdmissionRequest describes the admission.Attributes for the admission request. |Field|Description| |`uid`**[Required]**...
---

* [AdmissionReview](#admission-k8s-io-v1-AdmissionReview)
AdmissionRequest describes the admission.Attributes for the admission request.
|Field|Description|
|`uid`**[Required]**
[`k8s.io/apimachinery/pkg/types.UID`](https://pkg.go.dev/k8s.io/apimachinery/pkg/types#UID)|
uid is an identifier for the individual request/response. It allows us to distinguish instances of requests which are
otherwise identical (parallel requests, requests when earlier requests did not modify etc)