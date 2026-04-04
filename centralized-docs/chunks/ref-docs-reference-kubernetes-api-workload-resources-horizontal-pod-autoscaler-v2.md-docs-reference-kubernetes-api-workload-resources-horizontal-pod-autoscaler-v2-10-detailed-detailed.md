---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#10-detailed
chunk_level: detailed
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 1013
summary: * **desiredReplicas** (int32), required desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler. * **conditions**...
---

* **desiredReplicas** (int32), required
desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.
* **conditions** ([]HorizontalPodAutoscalerCondition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
*HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.*
* **conditions.status** (string), required
status is the status of the condition (True, False, Unknown)
* **conditions.type** (string), required
type describes the current condition
* **conditions.lastTransitionTime** (Time)
lastTransitionTime is the last time the condition transitioned from one status to another
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.message** (string)
message is a human-readable explanation containing details about the transition
* **conditions.reason** (string)
reason is the reason for the condition's last transition.
* **currentMetrics** ([]MetricStatus)
*Atomic: will be replaced during a merge*
currentMetrics is the last read state of the metrics used by this autoscaler.
*MetricStatus describes the last-read state of a single metric.*
* **currentMetrics.type** (string), required
type is the type of metric source. It will be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each corresponds to a matching field in the object.
* **currentMetrics.containerResource** (ContainerResourceMetricStatus)
container resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
*ContainerResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing a single container in each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.*
* **currentMetrics.containerResource.container** (string), required
container is the name of the container in the pods of the scaling target
* **currentMetrics.containerResource.current** (MetricValueStatus), required
current contains the current value for the given metric
*MetricValueStatus holds the current value for a metric*
* **currentMetrics.containerResource.current.averageUtilization** (int32)
currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
* **currentMetrics.containerResource.current.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
* **currentMetrics.containerResource.current.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the current value of the metric (as a quantity).
* **currentMetrics.containerResource.name** (string), required
name is the name of the resource in question.
* **currentMetrics.external** (ExternalMetricStatus)
external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
*ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.*
* **currentMetrics.external.current** (MetricValueStatus), required
current contains the current value for the given metric
*MetricValueStatus holds the current value for a metric*
* **currentMetrics.external.current.averageUtilization** (int32)
currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
* **currentMetrics.external.current.averageValue** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
* **currentMetrics.external.current.value** ([Quantity](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/#Quantity))
value is the current value of the metric (as a quantity).