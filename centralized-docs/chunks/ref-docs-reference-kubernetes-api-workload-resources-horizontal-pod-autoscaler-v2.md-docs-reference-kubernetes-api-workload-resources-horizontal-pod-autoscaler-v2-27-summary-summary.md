---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#27-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerSpec
token_count: 114
summary: * **behavior.scaleUp.policies** ([]HPAScalingPolicy) *Atomic: will be replaced during a merge* policies is a list of potential scaling polices which can be used during scaling. If not set, use the...
---

* **behavior.scaleUp.policies** ([]HPAScalingPolicy)
*Atomic: will be replaced during a merge*
policies is a list of potential scaling polices which can be used during scaling. If not set, use the default values: - For scale up: allow doubling the number of pods, or an absolute change of 4 pods in a 15s window. - For scale down: allow all pods to be removed in a 15s window.
*HPAScalingPolicy is a single policy which must hold true for a specified past interval.*