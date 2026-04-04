---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#58-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 89
summary: * **metrics.resource.name** (string), required name is the name of the resource in question. * **metrics.resource.target** (MetricTarget), required target specifies the target value for the given...
---

* **metrics.resource.name** (string), required
name is the name of the resource in question.
* **metrics.resource.target** (MetricTarget), required
target specifies the target value for the given metric
*MetricTarget defines the target value, average value, or average utilization of a specific metric*
* **metrics.resource.target.type** (string), required
type represents whether the metric type is Utilization, Value, or AverageValue