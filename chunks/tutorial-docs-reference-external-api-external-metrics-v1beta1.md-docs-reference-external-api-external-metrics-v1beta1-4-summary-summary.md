---
doc_id: tutorial/docs-reference-external-api-external-metrics-v1beta1.md/docs-reference-external-api-external-metrics-v1beta1
chunk_id: tutorial/docs-reference-external-api-external-metrics-v1beta1.md/docs-reference-external-api-external-metrics-v1beta1#4-summary
chunk_level: summary
chunk_type: table
heading: Resource Types
token_count: 125
summary: * [ExternalMetricValueList](#external-metrics-k8s-io-v1beta1-ExternalMetricValueList) ExternalMetricValue is a metric value for external metric A single metric value is identified by metric name and...
---

* [ExternalMetricValueList](#external-metrics-k8s-io-v1beta1-ExternalMetricValueList)
ExternalMetricValue is a metric value for external metric
A single metric value is identified by metric name and a set of string labels.
For one metric there can be multiple values with different sets of labels.
|Field|Description|
|`apiVersion`
string|`external.metrics.k8s.io/v1beta1`|
|`kind`
string|`ExternalMetricValue`|
|`metricName`**[Required]**
`string`|
the name of the metric
|
|