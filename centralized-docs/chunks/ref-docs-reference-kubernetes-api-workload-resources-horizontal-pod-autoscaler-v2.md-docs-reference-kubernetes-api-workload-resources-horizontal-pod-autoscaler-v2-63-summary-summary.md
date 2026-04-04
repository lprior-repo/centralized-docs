---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#63-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 120
summary: * **conditions.status** (string), required status is the status of the condition (True, False, Unknown) * **conditions.type** (string), required type describes the current condition *...
---

* **conditions.status** (string), required
status is the status of the condition (True, False, Unknown)
* **conditions.type** (string), required
type describes the current condition
* **conditions.lastTransitionTime** (Time)
lastTransitionTime is the last time the condition transitioned from one status to another
*Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON. Wrappers are provided for many of the factory methods that the time package offers.*
* **conditions.message** (string)
message is a human-readable explanation containing details about the transition