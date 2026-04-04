---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#52-summary
chunk_level: summary
chunk_type: prose
heading: Taint Nodes by Condition
token_count: 121
summary: The control plane, using the node [controller](/docs/concepts/architecture/controller/), automatically creates taints with a `NoSchedule` effect for [node...
---

The control plane, using the node [controller](/docs/concepts/architecture/controller/),
automatically creates taints with a `NoSchedule` effect for
[node conditions](/docs/concepts/scheduling-eviction/node-pressure-eviction/#node-conditions).
The scheduler checks taints, not node conditions, when it makes scheduling
decisions. This ensures that node conditions don't directly affect scheduling.
For example, if the `DiskPressure` node condition is active, the control plane
adds the `node.kubernetes.io/disk-pressure` taint and does not schedule new pods