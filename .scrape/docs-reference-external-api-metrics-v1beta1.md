---
url: https://kubernetes.io/docs/reference/external-api/metrics.v1beta1/
title: Kubernetes Metrics (v1beta1)
word_count: 372
filtered: true
elements_removed: 0
density_score: 0.85
---

## Table of Contents

- [Kubernetes Metrics (v1beta1)](#kubernetes-metrics-v1beta1)
  - [Resource Types](#resource-types)
  - [`NodeMetricsList`](#nodemetricslist)
  - [`PodMetrics`](#podmetrics)
  - [`PodMetricsList`](#podmetricslist)
  - [`ContainerMetrics`](#containermetrics)
  - [Feedback](#feedback)

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
## `PodMetricsList`
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
[`[]PodMetrics`](#metrics-k8s-io-v1beta1-PodMetrics)|
List of pod metrics.
|
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified May 18, 2023 at 9:35 PM PST: [Add config APIs metrics API (bc4758e3b2)](https://github.com/kubernetes/website/commit/bc4758e3b21d170a406cedc696d96f472de83759)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.
## Related Pages

- [Kubernetes External Metrics (v1beta1)](docs-reference-external-api-external-metrics-v1beta1.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
