---
doc_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1
chunk_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1#1-standard
chunk_level: standard
chunk_type: table
heading: Resource Types
token_count: 416
summary: # Kubernetes Metrics (v1beta1) Package v1beta1 is the v1beta1 version of the metrics API. ## Resource Types * [NodeMetrics](#metrics-k8s-io-v1beta1-NodeMetrics) *...
---

# Kubernetes Metrics (v1beta1)
Package v1beta1 is the v1beta1 version of the metrics API.
## Resource Types
* [NodeMetrics](#metrics-k8s-io-v1beta1-NodeMetrics)
* [NodeMetricsList](#metrics-k8s-io-v1beta1-NodeMetricsList)
* [PodMetrics](#metrics-k8s-io-v1beta1-PodMetrics)
* [PodMetricsList](#metrics-k8s-io-v1beta1-PodMetricsList)## `NodeMetrics`
**Appears in:**
* [NodeMetricsList](#metrics-k8s-io-v1beta1-NodeMetricsList)
NodeMetrics sets resource usage metrics of a node.
|Field|Description|
|`apiVersion`
string|`metrics.k8s.io/v1beta1`|
|`kind`
string|`NodeMetrics`|
|`metadata`
[`meta/v1.ObjectMeta`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#objectmeta-v1-meta)|
Standard object's metadata.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
Refer to the Kubernetes API documentation for the fields of the `metadata` field.|
|`timestamp`**[Required]**
[`meta/v1.Time`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#time-v1-meta)|
The following fields define time interval from which metrics were
collected from the interval [Timestamp-Window, Timestamp].
|
|`window`**[Required]**
[`meta/v1.Duration`](https://pkg.go.dev/k8s.io/apimachinery/pkg/apis/meta/v1#Duration)|No description provided.|
|`usage`**[Required]**
[`core/v1.ResourceList`](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#resourcelist-v1-core)|
The memory usage is the memory working set.
|