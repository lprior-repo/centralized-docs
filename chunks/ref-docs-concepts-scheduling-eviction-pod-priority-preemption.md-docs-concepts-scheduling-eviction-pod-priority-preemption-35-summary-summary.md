---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#35-summary
chunk_level: summary
chunk_type: prose
heading: Preemption
token_count: 123
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