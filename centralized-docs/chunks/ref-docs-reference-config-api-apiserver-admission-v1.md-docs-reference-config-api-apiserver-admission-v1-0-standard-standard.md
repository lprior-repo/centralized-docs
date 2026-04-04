---
doc_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1
chunk_id: ref/docs-reference-config-api-apiserver-admission-v1.md/docs-reference-config-api-apiserver-admission-v1#0-standard
chunk_level: standard
chunk_type: table
heading: Resource Types
token_count: 209
summary: ## Table of Contents    - [Resource Types](#resource-types)   - [`AdmissionRequest`](#admissionrequest)   - [`AdmissionResponse`](#admissionresponse)   - [`Operation`](#operation)   -...
---

## Table of Contents

  - [Resource Types](#resource-types)
  - [`AdmissionRequest`](#admissionrequest)
  - [`AdmissionResponse`](#admissionresponse)
  - [`Operation`](#operation)
  - [`PatchType`](#patchtype)
  - [Feedback](#feedback)

---

## Resource Types
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
response describes the attributes for the admission response.
|