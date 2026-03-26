---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#36-summary
chunk_level: summary
chunk_type: prose
heading: Preemption
token_count: 58
summary: #### Inter-Pod affinity on lower-priority Pods A Node is considered for preemption only when the answer to this question is yes: \"If all the Pods with lower priority than the pending Pod are removed...
---

#### Inter-Pod affinity on lower-priority Pods
A Node is considered for preemption only when the answer to this question is
yes: "If all the Pods with lower priority than the pending Pod are removed from
the Node, can the pending Pod be scheduled on the Node?"