---
doc_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1
chunk_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1#2-standard
chunk_level: standard
chunk_type: table
heading: `PodMetrics`
token_count: 457
summary: ## `NodeMetricsList` NodeMetricsList is a list of NodeMetrics. |Field|Description| |`apiVersion` string|`metrics.k8s.io/v1beta1`| |`kind` string|`NodeMetricsList`| |`metadata`**[Required]**...
---

## `NodeMetricsList`
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
[`[]NodeMetrics`](#metrics-k8s-io-v1beta1-NodeMetrics)|
List of node metrics.
|
## `PodMetrics`
**Appears in:**
* [PodMetricsList](#metrics-k8s-io-v1beta1-PodMetricsList)
PodMetrics sets resource usage metrics of a pod.
|Field|Description|
|`apiVersion`
string|`metrics.k8s.io/v1beta1`|
|`kind`
string|`PodMetrics`|
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
|`containers`**[Required]**
[`[]ContainerMetrics`](#metrics-k8s-io-v1beta1-ContainerMetrics)|
Metrics for all containers are collected within the same time window.
|