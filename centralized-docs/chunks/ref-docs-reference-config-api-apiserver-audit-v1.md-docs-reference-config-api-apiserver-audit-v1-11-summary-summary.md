---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#11-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 124
summary: Omitted for non-resource requests. Only logged at Request Level and higher. | |`responseObject`...
---

Omitted for non-resource requests. Only logged at Request Level and higher.
|
|`responseObject`
[`k8s.io/apimachinery/pkg/runtime.Unknown`](https://pkg.go.dev/k8s.io/apimachinery/pkg/runtime#Unknown)|
API object returned in the response, in JSON. The ResponseObject is recorded after conversion
to the external type, and serialized as JSON. Omitted for non-resource requests. Only logged
at Response Level.
|
|`requestReceivedTimestamp`
[`meta/v1.MicroTime`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#microtime-v1-meta)|