---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#4-standard
chunk_level: standard
chunk_type: table
heading: `EventList`
token_count: 316
summary: | |`requestReceivedTimestamp` [`meta/v1.MicroTime`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#microtime-v1-meta)| Time the request reached the apiserver. |...
---

|
|`requestReceivedTimestamp`
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
admission plugins. Note that these annotations are for the audit event, and do not correspond
to the metadata.annotations of the submitted object. Keys should uniquely identify the informing
component to avoid name collisions (e.g. podsecuritypolicy.admission.k8s.io/policy). Values
should be short. Annotations are included in the Metadata level.
|
## `EventList`
EventList is a list of audit Events.
|Field|Description|
|`apiVersion`
string|`audit.k8s.io/v1`|
|`kind`
string|`EventList`|
|`metadata`
[`meta/v1.ListMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.35/#listmeta-v1-meta)|No description provided.|
|`items`**[Required]**
[`[]Event`](#audit-k8s-io-v1-Event)|No description provided.|