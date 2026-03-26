---
doc_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1
chunk_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1#15-summary
chunk_level: summary
chunk_type: table
heading: `PodMetricsList`
token_count: 126
summary: PodMetricsList is a list of PodMetrics. |Field|Description| |`apiVersion` string|`metrics.k8s.io/v1beta1`| |`kind` string|`PodMetricsList`| |`metadata`**[Required]**...
---

PodMetricsList is a list of PodMetrics.
|Field|Description|
|`apiVersion`
string|`metrics.k8s.io/v1beta1`|
|`kind`
string|`PodMetricsList`|
|`metadata`**[Required]**
[`meta/v1.ListMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#listmeta-v1-meta)|
Standard list metadata.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
|
|`items`**[Required]**