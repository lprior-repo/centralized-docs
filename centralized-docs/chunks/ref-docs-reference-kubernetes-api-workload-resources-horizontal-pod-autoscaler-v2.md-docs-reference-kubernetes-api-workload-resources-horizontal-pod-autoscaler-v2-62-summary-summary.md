---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md/docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2#62-summary
chunk_level: summary
chunk_type: prose
heading: HorizontalPodAutoscalerStatus
token_count: 123
summary: * **desiredReplicas** (int32), required desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler. * **conditions**...
---

* **desiredReplicas** (int32), required
desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.
* **conditions** ([]HorizontalPodAutoscalerCondition)
*Patch strategy: merge on key `type`*
*Map: unique values on key type will be kept during a merge*
conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
*HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.*