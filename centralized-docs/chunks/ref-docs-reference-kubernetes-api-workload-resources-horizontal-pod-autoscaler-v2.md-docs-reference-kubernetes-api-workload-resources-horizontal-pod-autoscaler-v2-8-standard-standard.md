---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#8-standard
chunk_level: standard
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 501
summary: * **metrics** ([]MetricSpec) *Atomic: will be replaced during a merge* metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across...
---

* **metrics** ([]MetricSpec)
*Atomic: will be replaced during a merge*
metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across all metrics will be used). The desired replica count is calculated multiplying the ratio between the target value and the current value by the current number of pods. Ergo, metrics used must decrease as the pod count is increased, and vice-versa. See the individual metric source types for more information about how each type of metric must respond. If not set, the default metric will be set to 80% average CPU utilization.
*MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).*
* **metrics.type** (string), required
type is the type of metric source. It should be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each mapping to a matching field in the object.
* **metrics.containerResource** (ContainerResourceMetricSource)
containerResource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod of the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
*ContainerResourceMetricSource indicates how to scale on a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory). The values will be averaged together before being compared to the target. Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source. Only one "target" type should be set.*
* **metrics.containerResource.container** (string), required
container is the name of the container in the pods of the scaling target
* **metrics.containerResource.name** (string), required
name is the name of the resource in question.
* **metrics.containerResource.target** (MetricTarget), required
target specifies the target value for the given metric
*MetricTarget defines the target value, average value, or average utilization of a specific metric*
* **metrics.containerResource.target.type** (string), required
type represents whether the metric type is Utilization, Value, or AverageValue