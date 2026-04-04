---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#79-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 82
summary: * **currentMetrics.object.describedObject.apiVersion** (string) apiVersion is the API version of the referent * **currentMetrics.object.metric** (MetricIdentifier), required metric identifies the...
---

* **currentMetrics.object.describedObject.apiVersion** (string)
apiVersion is the API version of the referent
* **currentMetrics.object.metric** (MetricIdentifier), required
metric identifies the target metric by name and selector
*MetricIdentifier defines the name and optionally selector for a metric*
* **currentMetrics.object.metric.name** (string), required
name is the name of the given metric