---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#9-standard
chunk_level: standard
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 482
summary: * **metrics.containerResource.name** (string), required name is the name of the resource in question. * **metrics.containerResource.target** (MetricTarget), required target specifies the target value...
---

* **metrics.containerResource.name** (string), required
name is the name of the resource in question.
* **metrics.containerResource.target** (MetricTarget), required
target specifies the target value for the given metric
*MetricTarget defines the target value, average value, or average utilization of a specific metric*
* **metrics.containerResource.target.type** (string), required
type represents whether the metric type is Utilization, Value, or AverageValue
* **metrics.containerResource.target.averageUtilization** (int32)
averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type
* **metrics.containerResource.target.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
* **metrics.containerResource.target.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the target value of the metric (as a quantity).
* **metrics.external** (ExternalMetricSource)
external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
*ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).*
* **metrics.external.metric** (MetricIdentifier), required
metric identifies the target metric by name and selector
*MetricIdentifier defines the name and optionally selector for a metric*
* **metrics.external.metric.name** (string), required
name is the name of the given metric
* **metrics.external.metric.selector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.