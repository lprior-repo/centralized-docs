---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#15-standard
chunk_level: standard
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 363
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