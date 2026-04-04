---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#74-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 101
summary: * **currentMetrics.object** (ObjectMetricStatus) object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object). *ObjectMetricStatus indicates the...
---

* **currentMetrics.object** (ObjectMetricStatus)
object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
*ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).*
* **currentMetrics.object.current** (MetricValueStatus), required
current contains the current value for the given metric
*MetricValueStatus holds the current value for a metric*