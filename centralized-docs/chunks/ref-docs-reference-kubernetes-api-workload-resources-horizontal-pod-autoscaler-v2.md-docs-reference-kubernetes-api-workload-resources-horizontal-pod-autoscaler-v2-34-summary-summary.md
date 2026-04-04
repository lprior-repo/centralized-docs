---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#34-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 80
summary: *MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).* * **metrics.type** (string), required type is the type of metric source....
---

*MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).*
* **metrics.type** (string), required
type is the type of metric source. It should be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each mapping to a matching field in the object.