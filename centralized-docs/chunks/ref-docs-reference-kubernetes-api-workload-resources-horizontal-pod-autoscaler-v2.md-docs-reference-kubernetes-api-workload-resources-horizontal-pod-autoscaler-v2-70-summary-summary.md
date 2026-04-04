---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#70-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 105
summary: * **currentMetrics.containerResource.name** (string), required name is the name of the resource in question. * **currentMetrics.external** (ExternalMetricStatus) external refers to a global metric...
---

* **currentMetrics.containerResource.name** (string), required
name is the name of the resource in question.
* **currentMetrics.external** (ExternalMetricStatus)
external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
*ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.*