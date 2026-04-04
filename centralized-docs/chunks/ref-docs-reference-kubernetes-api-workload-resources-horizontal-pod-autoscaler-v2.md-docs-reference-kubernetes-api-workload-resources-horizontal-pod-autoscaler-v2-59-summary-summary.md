---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#59-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 83
summary: * **metrics.resource.target.type** (string), required type represents whether the metric type is Utilization, Value, or AverageValue * **metrics.resource.target.averageUtilization** (int32)...
---

* **metrics.resource.target.type** (string), required
type represents whether the metric type is Utilization, Value, or AverageValue
* **metrics.resource.target.averageUtilization** (int32)
averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type