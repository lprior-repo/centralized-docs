---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#12-summary
chunk_level: summary
chunk_type: prose
heading: Resource Types
token_count: 125
summary: [`meta/v1.MicroTime`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#microtime-v1-meta)| Time the request reached the apiserver. | |`stageTimestamp`...
---

[`meta/v1.MicroTime`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#microtime-v1-meta)|
Time the request reached the apiserver.
|
|`stageTimestamp`
[`meta/v1.MicroTime`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#microtime-v1-meta)|
Time the request reached current audit stage.
|
|`annotations`
`map[string]string`|
Annotations is an unstructured key value map stored with an audit event that may be set by
plugins invoked in the request serving chain, including authentication, authorization and