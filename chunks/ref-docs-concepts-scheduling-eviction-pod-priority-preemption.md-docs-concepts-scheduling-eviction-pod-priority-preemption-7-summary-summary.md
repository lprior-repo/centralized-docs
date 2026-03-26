---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#7-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 83
summary: # Pod Priority and Preemption FEATURE STATE: `Kubernetes v1.14 [stable]` [Pods](/docs/concepts/workloads/pods/) can have *priority*. Priority indicates the importance of a Pod relative to other Pods....
---

# Pod Priority and Preemption
FEATURE STATE:
`Kubernetes v1.14 [stable]`
[Pods](/docs/concepts/workloads/pods/) can have *priority*. Priority indicates the
importance of a Pod relative to other Pods. If a Pod cannot be scheduled, the
scheduler tries to preempt (evict) lower priority Pods to make scheduling of the
pending Pod possible.