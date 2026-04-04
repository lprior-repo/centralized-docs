---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#5-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 119
summary: [*Node affinity*](/docs/concepts/scheduling-eviction/assign-pod-node/#affinity-and-anti-affinity) is a property of [Pods](/docs/concepts/workloads/pods/) that *attracts* them to a set of...
---

[*Node affinity*](/docs/concepts/scheduling-eviction/assign-pod-node/#affinity-and-anti-affinity)
is a property of [Pods](/docs/concepts/workloads/pods/) that *attracts* them to
a set of [nodes](/docs/concepts/architecture/nodes/) (either as a preference or a
hard requirement). *Taints* are the opposite -- they allow a node to repel a set of pods.
*Tolerations* are applied to pods. Tolerations allow the scheduler to schedule pods with matching