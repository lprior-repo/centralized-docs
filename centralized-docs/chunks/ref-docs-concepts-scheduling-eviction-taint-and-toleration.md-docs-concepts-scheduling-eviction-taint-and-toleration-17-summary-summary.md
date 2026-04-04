---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#17-summary
chunk_level: summary
chunk_type: prose
heading: Concepts
token_count: 126
summary: \"preference\" or \"soft\" version of `NoSchedule`. The control plane will *try* to avoid placing a Pod that does not tolerate the taint on the node, but it is not guaranteed. You can put multiple taints...
---

"preference" or "soft" version of `NoSchedule`.
The control plane will *try* to avoid placing a Pod that does not tolerate
the taint on the node, but it is not guaranteed.
You can put multiple taints on the same node and multiple tolerations on the same pod.
The way Kubernetes processes multiple taints and tolerations is like a filter: start
with all of a node's taints, then ignore the ones for which the pod has a matching toleration; the
remaining un-ignored taints have the indicated effects on the pod. In particular,