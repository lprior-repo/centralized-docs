---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#54-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 127
summary: * **metrics.pods.target** (MetricTarget), required target specifies the target value for the given metric *MetricTarget defines the target value, average value, or average utilization of a specific...
---

* **metrics.pods.target** (MetricTarget), required
target specifies the target value for the given metric
*MetricTarget defines the target value, average value, or average utilization of a specific metric*
* **metrics.pods.target.type** (string), required
type represents whether the metric type is Utilization, Value, or AverageValue
* **metrics.pods.target.averageUtilization** (int32)
averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type