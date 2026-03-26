---
doc_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1
chunk_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1#17-summary
chunk_level: summary
chunk_type: table
heading: `ContainerMetrics`
token_count: 121
summary: ## `ContainerMetrics` **Appears in:** * [PodMetrics](#metrics-k8s-io-v1beta1-PodMetrics) ContainerMetrics sets resource usage metrics of a container. |Field|Description| |`name`**[Required]**...
---

## `ContainerMetrics`
**Appears in:**
* [PodMetrics](#metrics-k8s-io-v1beta1-PodMetrics)
ContainerMetrics sets resource usage metrics of a container.
|Field|Description|
|`name`**[Required]**
`string`|
Container name corresponding to the one from pod.spec.containers.
|
|`usage`**[Required]**
[`core/v1.ResourceList`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#resourcelist-v1-core)|
The memory usage is the memory working set.
|