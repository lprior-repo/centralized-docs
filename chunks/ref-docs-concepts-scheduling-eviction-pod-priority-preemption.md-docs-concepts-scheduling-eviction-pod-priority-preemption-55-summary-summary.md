---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#55-summary
chunk_level: summary
chunk_type: prose
heading: Interactions between Pod priority and quality of service
token_count: 127
summary: Pod priority and [QoS class](/docs/concepts/workloads/pods/pod-qos/) are two orthogonal features with few interactions and no default restrictions on setting the priority of a Pod based on its QoS...
---

Pod priority and [QoS class](/docs/concepts/workloads/pods/pod-qos/)
are two orthogonal features with few interactions and no default restrictions on
setting the priority of a Pod based on its QoS classes. The scheduler's
preemption logic does not consider QoS when choosing preemption targets.
Preemption considers Pod priority and attempts to choose a set of targets with
the lowest priority. Higher-priority Pods are considered for preemption only if
the removal of the lowest priority Pods is not sufficient to allow the scheduler
to schedule the preemptor Pod, or if the lowest priority Pods are protected by