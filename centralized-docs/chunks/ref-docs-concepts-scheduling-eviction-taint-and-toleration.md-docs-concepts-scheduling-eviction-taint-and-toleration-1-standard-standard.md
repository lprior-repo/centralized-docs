---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 227
summary: # Taints and Tolerations [*Node affinity*](/docs/concepts/scheduling-eviction/assign-pod-node/#affinity-and-anti-affinity) is a property of [Pods](/docs/concepts/workloads/pods/) that *attracts* them...
---

# Taints and Tolerations
[*Node affinity*](/docs/concepts/scheduling-eviction/assign-pod-node/#affinity-and-anti-affinity)
is a property of [Pods](/docs/concepts/workloads/pods/) that *attracts* them to
a set of [nodes](/docs/concepts/architecture/nodes/) (either as a preference or a
hard requirement). *Taints* are the opposite -- they allow a node to repel a set of pods.
*Tolerations* are applied to pods. Tolerations allow the scheduler to schedule pods with matching
taints. Tolerations allow scheduling but don't guarantee scheduling: the scheduler also
[evaluates other parameters](/docs/concepts/scheduling-eviction/pod-priority-preemption/)
as part of its function.
Taints and tolerations work together to ensure that pods are not scheduled
onto inappropriate nodes. One or more taints are applied to a node; this
marks that the node should not accept any pods that do not tolerate the taints.