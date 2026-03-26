---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#20-summary
chunk_level: summary
chunk_type: prose
heading: Non-preempting PriorityClass
token_count: 97
summary: is set to `Never`, pods in that PriorityClass will be non-preempting. An example use case is for data science workloads. A user may submit a job that they want to be prioritized above other...
---

 is set to `Never`,
pods in that PriorityClass will be non-preempting.
An example use case is for data science workloads.
A user may submit a job that they want to be prioritized above other workloads,
but do not wish to discard existing work by preempting running pods.
The high priority job with `preemptionPolicy: Never` will be scheduled
ahead of other queued pods,
as soon as sufficient cluster resources "naturally" become free.