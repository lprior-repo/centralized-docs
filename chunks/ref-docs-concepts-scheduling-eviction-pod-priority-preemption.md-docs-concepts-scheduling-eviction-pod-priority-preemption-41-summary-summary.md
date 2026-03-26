---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#41-summary
chunk_level: summary
chunk_type: prose
heading: Preemption
token_count: 72
summary: * Pod P is being considered for Node N. * Pod Q is running on another Node in the same Zone as Node N. * Pod P has Zone-wide anti-affinity with Pod Q (`topologyKey: topology.kubernetes.io/zone`). *...
---

* Pod P is being considered for Node N.
* Pod Q is running on another Node in the same Zone as Node N.
* Pod P has Zone-wide anti-affinity with Pod Q (`topologyKey: topology.kubernetes.io/zone`).
* There are no other cases of anti-affinity between Pod P and other Pods in
the Zone.