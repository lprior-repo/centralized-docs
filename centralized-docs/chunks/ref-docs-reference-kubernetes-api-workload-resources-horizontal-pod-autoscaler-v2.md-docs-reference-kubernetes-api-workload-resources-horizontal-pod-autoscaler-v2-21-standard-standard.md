---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#21-standard
chunk_level: standard
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 334
summary: * **currentMetrics.resource.current** (MetricValueStatus), required current contains the current value for the given metric *MetricValueStatus holds the current value for a metric* *...
---

* **currentMetrics.resource.current** (MetricValueStatus), required
current contains the current value for the given metric
*MetricValueStatus holds the current value for a metric*
* **currentMetrics.resource.current.averageUtilization** (int32)
currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
* **currentMetrics.resource.current.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
* **currentMetrics.resource.current.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the current value of the metric (as a quantity).
* **currentMetrics.resource.name** (string), required
name is the name of the resource in question.
* **currentReplicas** (int32)
currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler.
* **lastScaleTime** (Time)
lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed.
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **observedGeneration** (int64)
observedGeneration is the most recent generation observed by this autoscaler.