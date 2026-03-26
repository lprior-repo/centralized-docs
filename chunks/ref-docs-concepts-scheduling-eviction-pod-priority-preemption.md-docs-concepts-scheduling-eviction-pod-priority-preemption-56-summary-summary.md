---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#56-summary
chunk_level: summary
chunk_type: prose
heading: Interactions between Pod priority and quality of service
token_count: 88
summary: to schedule the preemptor Pod, or if the lowest priority Pods are protected by `PodDisruptionBudget`. The kubelet uses Priority to determine pod order for [node-pressure...
---

to schedule the preemptor Pod, or if the lowest priority Pods are protected by
`PodDisruptionBudget`.
The kubelet uses Priority to determine pod order for [node-pressure eviction](/docs/concepts/scheduling-eviction/node-pressure-eviction/).
You can use the QoS class to estimate the order in which pods are most likely
to get evicted. The kubelet ranks pods for eviction based on the following factors: