---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#15-standard
chunk_level: standard
chunk_type: prose
heading: Taint Nodes by Condition
token_count: 403
summary: ## Taint Nodes by Condition The control plane, using the node [controller](/docs/concepts/architecture/controller/), automatically creates taints with a `NoSchedule` effect for [node...
---

## Taint Nodes by Condition
The control plane, using the node [controller](/docs/concepts/architecture/controller/),
automatically creates taints with a `NoSchedule` effect for
[node conditions](/docs/concepts/scheduling-eviction/node-pressure-eviction/#node-conditions).
The scheduler checks taints, not node conditions, when it makes scheduling
decisions. This ensures that node conditions don't directly affect scheduling.
For example, if the `DiskPressure` node condition is active, the control plane
adds the `node.kubernetes.io/disk-pressure` taint and does not schedule new pods
onto the affected node. If the `MemoryPressure` node condition is active, the
control plane adds the `node.kubernetes.io/memory-pressure` taint.
You can ignore node conditions for newly created pods by adding the corresponding
Pod tolerations. The control plane also adds the `node.kubernetes.io/memory-pressure`
toleration on pods that have a [QoS class](/docs/concepts/workloads/pods/pod-qos/)
other than `BestEffort`. This is because Kubernetes treats pods in the `Guaranteed`
or `Burstable` QoS classes (even pods with no memory request set) as if they are
able to cope with memory pressure, while new `BestEffort` pods are not scheduled
onto the affected node.
The DaemonSet controller automatically adds the following `NoSchedule`
tolerations to all daemons, to prevent DaemonSets from breaking.
* `node.kubernetes.io/memory-pressure`
* `node.kubernetes.io/disk-pressure`
* `node.kubernetes.io/pid-pressure` (1.14 or later)
* `node.kubernetes.io/unschedulable` (1.10 or later)
* `node.kubernetes.io/network-unavailable` (*host network only*)
Adding these tolerations ensures backward compatibility. You can also add
arbitrary tolerations to DaemonSets.