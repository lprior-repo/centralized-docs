---
doc_id: tutorial/docs-reference-external-api-external-metrics-v1beta1.md/docs-reference-external-api-external-metrics-v1beta1
chunk_id: tutorial/docs-reference-external-api-external-metrics-v1beta1.md/docs-reference-external-api-external-metrics-v1beta1#5-summary
chunk_level: summary
chunk_type: table
heading: Resource Types
token_count: 127
summary: string|`ExternalMetricValue`| |`metricName`**[Required]** `string`| the name of the metric | |`metricLabels`**[Required]** `map[string]string`| a set of labels that identify a single time series for...
---

string|`ExternalMetricValue`|
|`metricName`**[Required]**
`string`|
the name of the metric
|
|`metricLabels`**[Required]**
`map[string]string`|
a set of labels that identify a single time series for the metric
|
|`timestamp`**[Required]**
[`meta/v1.Time`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#time-v1-meta)|
indicates the time at which the metrics were produced
|
|`window`**[Required]**
`int64`|
indicates the window ([