---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#11-detailed
chunk_level: detailed
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 1003
summary: * **currentMetrics.external.current** (MetricValueStatus), required current contains the current value for the given metric *MetricValueStatus holds the current value for a metric* *...
---

* **currentMetrics.external.current** (MetricValueStatus), required
current contains the current value for the given metric
*MetricValueStatus holds the current value for a metric*
* **currentMetrics.external.current.averageUtilization** (int32)
currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
* **currentMetrics.external.current.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
* **currentMetrics.external.current.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the current value of the metric (as a quantity).
* **currentMetrics.external.metric** (MetricIdentifier), required
metric identifies the target metric by name and selector
*MetricIdentifier defines the name and optionally selector for a metric*
* **currentMetrics.external.metric.name** (string), required
name is the name of the given metric
* **currentMetrics.external.metric.selector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
* **currentMetrics.object** (ObjectMetricStatus)
object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
*ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).*
* **currentMetrics.object.current** (MetricValueStatus), required
current contains the current value for the given metric
*MetricValueStatus holds the current value for a metric*
* **currentMetrics.object.current.averageUtilization** (int32)
currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
* **currentMetrics.object.current.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
* **currentMetrics.object.current.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the current value of the metric (as a quantity).
* **currentMetrics.object.describedObject** (CrossVersionObjectReference), required
DescribedObject specifies the descriptions of a object,such as kind,name apiVersion
*CrossVersionObjectReference contains enough information to let you identify the referred resource.*
* **currentMetrics.object.describedObject.kind** (string), required
kind is the kind of the referent; More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds)
* **currentMetrics.object.describedObject.name** (string), required
name is the name of the referent; More info: [https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names](https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names)
* **currentMetrics.object.describedObject.apiVersion** (string)
apiVersion is the API version of the referent
* **currentMetrics.object.metric** (MetricIdentifier), required
metric identifies the target metric by name and selector
*MetricIdentifier defines the name and optionally selector for a metric*
* **currentMetrics.object.metric.name** (string), required
name is the name of the given metric
* **currentMetrics.object.metric.selector** ([LabelSelector](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/#LabelSelector))
selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
* **currentMetrics.pods** (PodsMetricStatus)
pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.
*PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).*