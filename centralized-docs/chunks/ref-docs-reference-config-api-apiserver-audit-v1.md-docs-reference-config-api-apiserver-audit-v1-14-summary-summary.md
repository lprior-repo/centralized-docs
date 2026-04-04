---
doc_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1
chunk_id: ref/docs-reference-config-api-apiserver-audit-v1.md/docs-reference-config-api-apiserver-audit-v1#14-summary
chunk_level: summary
chunk_type: table
heading: `EventList`
token_count: 112
summary: ## `EventList` EventList is a list of audit Events. |Field|Description| |`apiVersion` string|`audit.k8s.io/v1`| |`kind` string|`EventList`| |`metadata`...
---

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