---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#7-detailed
chunk_level: detailed
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 897
summary: * **metrics.object.describedObject.name** (string), required name is the name of the referent; More info:...
---

* **metrics.object.describedObject.name** (string), required
name is the name of the referent; More info: [https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names](https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names)
* **metrics.object.describedObject.apiVersion** (string)
apiVersion is the API version of the referent
* **metrics.object.metric** (MetricIdentifier), required
metric identifies the target metric by name and selector
*MetricIdentifier defines the name and optionally selector for a metric*
* **metrics.object.metric.name** (string), required
name is the name of the given metric
* **metrics.object.metric.selector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
* **metrics.object.target** (MetricTarget), required
target specifies the target value for the given metric
*MetricTarget defines the target value, average value, or average utilization of a specific metric*
* **metrics.object.target.type** (string), required
type represents whether the metric type is Utilization, Value, or AverageValue
* **metrics.object.target.averageUtilization** (int32)
averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type
* **metrics.object.target.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
* **metrics.object.target.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the target value of the metric (as a quantity).
* **metrics.pods** (PodsMetricSource)
pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.
*PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.*
* **metrics.pods.metric** (MetricIdentifier), required
metric identifies the target metric by name and selector
*MetricIdentifier defines the name and optionally selector for a metric*
* **metrics.pods.metric.name** (string), required
name is the name of the given metric
* **metrics.pods.metric.selector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
* **metrics.pods.target** (MetricTarget), required
target specifies the target value for the given metric
*MetricTarget defines the target value, average value, or average utilization of a specific metric*
* **metrics.pods.target.type** (string), required
type represents whether the metric type is Utilization, Value, or AverageValue
* **metrics.pods.target.averageUtilization** (int32)
averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type
* **metrics.pods.target.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
* **metrics.pods.target.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the target value of the metric (as a quantity).