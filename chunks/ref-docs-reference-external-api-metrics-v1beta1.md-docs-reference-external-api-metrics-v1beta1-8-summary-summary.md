---
doc_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1
chunk_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1#8-summary
chunk_level: summary
chunk_type: table
heading: `NodeMetricsList`
token_count: 126
summary: NodeMetricsList is a list of NodeMetrics. |Field|Description| |`apiVersion` string|`metrics.k8s.io/v1beta1`| |`kind` string|`NodeMetricsList`| |`metadata`**[Required]**...
---

NodeMetricsList is a list of NodeMetrics.
|Field|Description|
|`apiVersion`
string|`metrics.k8s.io/v1beta1`|
|`kind`
string|`NodeMetricsList`|
|`metadata`**[Required]**
[`meta/v1.ListMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#listmeta-v1-meta)|
Standard list metadata.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
|
|`items`**[Required]**