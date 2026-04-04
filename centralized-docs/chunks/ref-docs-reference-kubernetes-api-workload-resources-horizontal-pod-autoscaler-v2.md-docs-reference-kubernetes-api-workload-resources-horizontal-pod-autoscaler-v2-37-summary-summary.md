---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#37-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 119
summary: * **metrics.containerResource.container** (string), required container is the name of the container in the pods of the scaling target * **metrics.containerResource.name** (string), required name is...
---

* **metrics.containerResource.container** (string), required
container is the name of the container in the pods of the scaling target
* **metrics.containerResource.name** (string), required
name is the name of the resource in question.
* **metrics.containerResource.target** (MetricTarget), required
target specifies the target value for the given metric
*MetricTarget defines the target value, average value, or average utilization of a specific metric*
* **metrics.containerResource.target.type** (string), required
type represents whether the metric type is Utilization, Value, or AverageValue