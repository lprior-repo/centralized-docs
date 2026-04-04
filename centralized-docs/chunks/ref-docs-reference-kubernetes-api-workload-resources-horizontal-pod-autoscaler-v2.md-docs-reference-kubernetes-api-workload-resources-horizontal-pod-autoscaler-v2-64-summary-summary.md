---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#64-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 87
summary: * **conditions.message** (string) message is a human-readable explanation containing details about the transition * **conditions.reason** (string) reason is the reason for the condition's last...
---

* **conditions.message** (string)
message is a human-readable explanation containing details about the transition
* **conditions.reason** (string)
reason is the reason for the condition's last transition.
* **currentMetrics** ([]MetricStatus)
*Atomic: will be replaced during a merge*
currentMetrics is the last read state of the metrics used by this autoscaler.
*MetricStatus describes the last-read state of a single metric.*