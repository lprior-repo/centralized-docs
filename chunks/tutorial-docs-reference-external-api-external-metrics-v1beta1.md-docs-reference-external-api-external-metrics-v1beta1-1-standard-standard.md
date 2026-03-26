---
doc_id: tutorial/docs-reference-external-api-external-metrics-v1beta1.md/docs-reference-external-api-external-metrics-v1beta1
chunk_id: tutorial/docs-reference-external-api-external-metrics-v1beta1.md/docs-reference-external-api-external-metrics-v1beta1#1-standard
chunk_level: standard
chunk_type: table
heading: Resource Types
token_count: 393
summary: # Kubernetes External Metrics (v1beta1) Package v1beta1 is the v1beta1 version of the external metrics API. ## Resource Types *...
---

# Kubernetes External Metrics (v1beta1)
Package v1beta1 is the v1beta1 version of the external metrics API.
## Resource Types
* [ExternalMetricValue](#external-metrics-k8s-io-v1beta1-ExternalMetricValue)
* [ExternalMetricValueList](#external-metrics-k8s-io-v1beta1-ExternalMetricValueList)## `ExternalMetricValue`
**Appears in:**
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
indicates the window ([Timestamp-Window, Timestamp]) from
which these metrics were calculated, when returning rate
metrics calculated from cumulative metrics (or zero for
non-calculated instantaneous metrics).
|
|`value`**[Required]**
[`k8s.io/apimachinery/pkg/api/resource.Quantity`](https://pkg.go.dev/k8s.io/apimachinery/pkg/api/resource#Quantity)|
the value of the metric
|