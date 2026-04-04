---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#55-summary
chunk_level: summary
chunk_type: prose
heading: Taint Nodes by Condition
token_count: 91
summary: * `node.kubernetes.io/memory-pressure` * `node.kubernetes.io/disk-pressure` * `node.kubernetes.io/pid-pressure` (1.14 or later) * `node.kubernetes.io/unschedulable` (1.10 or later) *...
---

* `node.kubernetes.io/memory-pressure`
* `node.kubernetes.io/disk-pressure`
* `node.kubernetes.io/pid-pressure` (1.14 or later)
* `node.kubernetes.io/unschedulable` (1.10 or later)
* `node.kubernetes.io/network-unavailable` (*host network only*)
Adding these tolerations ensures backward compatibility. You can also add
arbitrary tolerations to DaemonSets.