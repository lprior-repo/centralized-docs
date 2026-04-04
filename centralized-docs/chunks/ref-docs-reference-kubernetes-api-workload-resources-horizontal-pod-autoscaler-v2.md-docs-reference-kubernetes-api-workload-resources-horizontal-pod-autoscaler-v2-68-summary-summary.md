---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#68-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 115
summary: * **currentMetrics.containerResource.container** (string), required container is the name of the container in the pods of the scaling target * **currentMetrics.containerResource.current**...
---

* **currentMetrics.containerResource.container** (string), required
container is the name of the container in the pods of the scaling target
* **currentMetrics.containerResource.current** (MetricValueStatus), required
current contains the current value for the given metric
*MetricValueStatus holds the current value for a metric*
* **currentMetrics.containerResource.current.averageUtilization** (int32)
currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.