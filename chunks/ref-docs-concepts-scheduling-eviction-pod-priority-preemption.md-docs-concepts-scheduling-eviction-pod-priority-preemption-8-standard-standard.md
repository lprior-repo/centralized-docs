---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#8-standard
chunk_level: standard
chunk_type: prose
heading: Preemption
token_count: 230
summary: #### Cross node preemption Suppose a Node N is being considered for preemption so that a pending Pod P can be scheduled on N. P might become feasible on N only if a Pod on another Node is preempted....
---

#### Cross node preemption
Suppose a Node N is being considered for preemption so that a pending Pod P can
be scheduled on N. P might become feasible on N only if a Pod on another Node is
preempted. Here's an example:
* Pod P is being considered for Node N.
* Pod Q is running on another Node in the same Zone as Node N.
* Pod P has Zone-wide anti-affinity with Pod Q (`topologyKey: topology.kubernetes.io/zone`).
* There are no other cases of anti-affinity between Pod P and other Pods in
the Zone.
* In order to schedule Pod P on Node N, Pod Q can be preempted, but scheduler
does not perform cross-node preemption. So, Pod P will be deemed
unschedulable on Node N.
If Pod Q were removed from its Node, the Pod anti-affinity violation would be
gone, and Pod P could possibly be scheduled on Node N.
We may consider adding cross Node preemption in future versions if there is
enough demand and if we find an algorithm with reasonable performance.