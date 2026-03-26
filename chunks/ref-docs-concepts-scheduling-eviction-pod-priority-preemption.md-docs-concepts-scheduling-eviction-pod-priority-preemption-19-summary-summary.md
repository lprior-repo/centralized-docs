---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#19-summary
chunk_level: summary
chunk_type: prose
heading: Non-preempting PriorityClass
token_count: 125
summary: This means that if the scheduler tries these pods and they cannot be scheduled, they will be retried with lower frequency, allowing other pods with lower priority to be scheduled before them....
---

This means that if the scheduler tries these pods and they cannot be scheduled,
they will be retried with lower frequency,
allowing other pods with lower priority to be scheduled before them.
Non-preempting pods may still be preempted by other,
high-priority pods.
`preemptionPolicy` defaults to `PreemptLowerPriority`,
which will allow pods of that PriorityClass to preempt lower-priority pods
(as is existing default behavior).
If `preemptionPolicy` is set to `Never`,
pods in that PriorityClass will be non-preempting.
An example use case is for data science workloads.