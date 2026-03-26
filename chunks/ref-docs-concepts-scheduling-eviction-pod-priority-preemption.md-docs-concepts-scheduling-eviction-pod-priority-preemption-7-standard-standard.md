---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#7-standard
chunk_level: standard
chunk_type: prose
heading: Preemption
token_count: 399
summary: #### PodDisruptionBudget is supported, but not guaranteed A [PodDisruptionBudget](/docs/concepts/workloads/pods/disruptions/) (PDB) allows application owners to limit the number of Pods of a...
---

#### PodDisruptionBudget is supported, but not guaranteed
A [PodDisruptionBudget](/docs/concepts/workloads/pods/disruptions/) (PDB)
allows application owners to limit the number of Pods of a replicated application
that are down simultaneously from voluntary disruptions. Kubernetes supports
PDB when preempting Pods, but respecting PDB is best effort. The scheduler tries
to find victims whose PDB are not violated by preemption, but if no such victims
are found, preemption will still happen, and lower priority Pods will be removed
despite their PDBs being violated.
#### Inter-Pod affinity on lower-priority Pods
A Node is considered for preemption only when the answer to this question is
yes: "If all the Pods with lower priority than the pending Pod are removed from
the Node, can the pending Pod be scheduled on the Node?"
#### Note:
Preemption does not necessarily remove all lower-priority
Pods. If the pending Pod can be scheduled by removing fewer than all
lower-priority Pods, then only a portion of the lower-priority Pods are removed.
Even so, the answer to the preceding question must be yes. If the answer is no,
the Node is not considered for preemption.
If a pending Pod has inter-pod [affinity](/docs/concepts/scheduling-eviction/assign-pod-node/#affinity-and-anti-affinity)
to one or more of the lower-priority Pods on the Node, the inter-Pod affinity
rule cannot be satisfied in the absence of those lower-priority Pods. In this case,
the scheduler does not preempt any Pods on the Node. Instead, it looks for another
Node. The scheduler might find a suitable Node or it might not. There is no
guarantee that the pending Pod can be scheduled.
Our recommended solution for this problem is to create inter-Pod affinity only
towards equal or higher priority Pods.