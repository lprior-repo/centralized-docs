---
doc_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1
chunk_id: ref/docs-reference-external-api-metrics-v1beta1.md/docs-reference-external-api-metrics-v1beta1#2-detailed
chunk_level: detailed
chunk_type: table
heading: Related Pages
token_count: 652
summary: ## `PodMetricsList` PodMetricsList is a list of PodMetrics. |Field|Description| |`apiVersion` string|`metrics.k8s.io/v1beta1`| |`kind` string|`PodMetricsList`| |`metadata`**[Required]**...
---

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