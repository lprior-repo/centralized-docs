---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#4-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 128
summary: | AuditLevel at which event was generated | |`auditID`**[Required]** [`k8s.io/apimachinery/pkg/types.UID`](https://pkg.go.dev/k8s.io/apimachinery/pkg/types#UID)| Unique audit ID, generated for each...
---

|
AuditLevel at which event was generated
|
|`auditID`**[Required]**
[`k8s.io/apimachinery/pkg/types.UID`](https://pkg.go.dev/k8s.io/apimachinery/pkg/types#UID)|
Unique audit ID, generated for each request.
|
|`stage`**[Required]**
[`Stage`](#audit-k8s-io-v1-Stage)|
Stage of the request handling when this event instance was generated.
|
|`requestURI`**[Required]**
`string`|
RequestURI is the request URI as sent by the client to a server.
|
|`verb`