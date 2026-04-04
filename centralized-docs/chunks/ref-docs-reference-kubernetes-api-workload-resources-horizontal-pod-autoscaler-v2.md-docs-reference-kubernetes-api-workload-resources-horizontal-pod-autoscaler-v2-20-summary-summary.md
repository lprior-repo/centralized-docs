---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#20-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 109
summary: * **behavior.scaleDown.policies.type** (string), required type is used to specify the scaling policy. * **behavior.scaleDown.policies.value** (int32), required value contains the amount of change...
---

* **behavior.scaleDown.policies.type** (string), required
type is used to specify the scaling policy.
* **behavior.scaleDown.policies.value** (int32), required
value contains the amount of change which is permitted by the policy. It must be greater than zero
* **behavior.scaleDown.policies.periodSeconds** (int32), required
periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).