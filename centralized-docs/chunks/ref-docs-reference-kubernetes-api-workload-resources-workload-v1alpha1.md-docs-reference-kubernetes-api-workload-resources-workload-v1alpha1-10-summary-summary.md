---
doc_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1
chunk_id: ref/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1.md/docs-reference-kubernetes-api-workload-resources-workload-v1alpha1#10-summary
chunk_level: summary
chunk_type: prose
heading: WorkloadSpec
token_count: 128
summary: * **podGroups.policy** (PodGroupPolicy), required Policy defines the scheduling policy for this PodGroup. *PodGroupPolicy defines the scheduling configuration for a PodGroup.* *...
---

* **podGroups.policy** (PodGroupPolicy), required
Policy defines the scheduling policy for this PodGroup.
*PodGroupPolicy defines the scheduling configuration for a PodGroup.*
* **podGroups.policy.basic** (BasicSchedulingPolicy)
Basic specifies that the pods in this group should be scheduled using standard Kubernetes scheduling behavior.
*BasicSchedulingPolicy indicates that standard Kubernetes scheduling behavior should be used.*
* **podGroups.policy.gang** (GangSchedulingPolicy)
Gang specifies that the pods in this group should be scheduled using all-or-nothing semantics.
*GangSchedulingPolicy defines the parameters for gang scheduling.*