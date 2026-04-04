---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#10-standard
chunk_level: standard
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 488
summary: * **metrics.external.metric.selector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector)) selector is the string-encoded form of a...
---

* **metrics.external.metric.selector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
* **metrics.external.target** (MetricTarget), required
target specifies the target value for the given metric
*MetricTarget defines the target value, average value, or average utilization of a specific metric*
* **metrics.external.target.type** (string), required
type represents whether the metric type is Utilization, Value, or AverageValue
* **metrics.external.target.averageUtilization** (int32)
averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type
* **metrics.external.target.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
* **metrics.external.target.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the target value of the metric (as a quantity).
* **metrics.object** (ObjectMetricSource)
object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
*ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).*
* **metrics.object.describedObject** (CrossVersionObjectReference), required
describedObject specifies the descriptions of a object,such as kind,name apiVersion
*CrossVersionObjectReference contains enough information to let you identify the referred resource.*
* **metrics.object.describedObject.kind** (string), required
kind is the kind of the referent; More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)