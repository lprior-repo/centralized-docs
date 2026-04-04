---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#45-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 113
summary: * **metrics.object** (ObjectMetricSource) object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object). *ObjectMetricSource indicates how to...
---

* **metrics.object** (ObjectMetricSource)
object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
*ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).*
* **metrics.object.describedObject** (CrossVersionObjectReference), required
describedObject specifies the descriptions of a object,such as kind,name apiVersion
*CrossVersionObjectReference contains enough information to let you identify the referred resource.*