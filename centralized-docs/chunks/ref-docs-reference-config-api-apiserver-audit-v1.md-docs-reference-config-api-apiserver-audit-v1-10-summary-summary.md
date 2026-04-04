---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#10-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 128
summary: For non-status type error responses, this will be auto-populated with the error Message. | |`requestObject`...
---

For non-status type error responses, this will be auto-populated with the error Message.
|
|`requestObject`
[`k8s.io/apimachinery/pkg/runtime.Unknown`](https://pkg.go.dev/k8s.io/apimachinery/pkg/runtime#Unknown)|
API object from the request, in JSON format. The RequestObject is recorded as-is in the request
(possibly re-encoded as JSON), prior to version conversion, defaulting, admission or
merging. It is an external versioned object type, and may not be a valid object on its own.
Omitted for non-resource requests. Only logged at Request Level and higher.
|