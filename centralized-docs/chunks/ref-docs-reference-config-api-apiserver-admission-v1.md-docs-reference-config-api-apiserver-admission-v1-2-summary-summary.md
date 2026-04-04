---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#2-summary
chunk_level: summary
chunk_type: table
heading: Resource Types
token_count: 128
summary: * [AdmissionReview](#admission-k8s-io-v1-AdmissionReview)## `AdmissionReview` AdmissionReview describes an admission review request/response. |Field|Description| |`apiVersion`...
---

* [AdmissionReview](#admission-k8s-io-v1-AdmissionReview)## `AdmissionReview`
AdmissionReview describes an admission review request/response.
|Field|Description|
|`apiVersion`
string|`admission.k8s.io/v1`|
|`kind`
string|`AdmissionReview`|
|`request`
[`AdmissionRequest`](#admission-k8s-io-v1-AdmissionRequest)|
request describes the attributes for the admission request.
|
|`response`
[`AdmissionResponse`](#admission-k8s-io-v1-AdmissionResponse)|