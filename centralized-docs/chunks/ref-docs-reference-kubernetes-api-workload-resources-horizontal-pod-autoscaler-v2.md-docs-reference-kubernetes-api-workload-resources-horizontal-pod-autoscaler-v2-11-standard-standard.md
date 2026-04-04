---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#11-standard
chunk_level: standard
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 482
summary: * **metrics.object.describedObject.kind** (string), required kind is the kind of the referent; More info:...
---

* **metrics.object.describedObject.kind** (string), required
kind is the kind of the referent; More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)
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