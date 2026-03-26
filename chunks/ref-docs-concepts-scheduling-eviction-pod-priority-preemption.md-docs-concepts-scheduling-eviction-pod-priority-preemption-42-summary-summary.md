---
doc_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption
chunk_id: ref/docs-concepts-scheduling-eviction-pod-priority-preemption.md/docs-concepts-scheduling-eviction-pod-priority-preemption#42-summary
chunk_level: summary
chunk_type: prose
heading: Preemption
token_count: 125
summary: * There are no other cases of anti-affinity between Pod P and other Pods in the Zone. * In order to schedule Pod P on Node N, Pod Q can be preempted, but scheduler does not perform cross-node...
---

* There are no other cases of anti-affinity between Pod P and other Pods in
the Zone.
* In order to schedule Pod P on Node N, Pod Q can be preempted, but scheduler
does not perform cross-node preemption. So, Pod P will be deemed
unschedulable on Node N.
If Pod Q were removed from its Node, the Pod anti-affinity violation would be
gone, and Pod P could possibly be scheduled on Node N.
We may consider adding cross Node preemption in future versions if there is
enough demand and if we find an algorithm with reasonable performance.