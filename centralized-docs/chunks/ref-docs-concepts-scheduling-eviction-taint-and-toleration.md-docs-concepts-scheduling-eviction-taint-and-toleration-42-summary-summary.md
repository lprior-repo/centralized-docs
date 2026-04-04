---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#42-summary
chunk_level: summary
chunk_type: prose
heading: Taint based Evictions
token_count: 121
summary: * `node.kubernetes.io/not-ready`: Node is not ready. This corresponds to the NodeCondition `Ready` being \"`False`\". * `node.kubernetes.io/unreachable`: Node is unreachable from the node controller....
---

* `node.kubernetes.io/not-ready`: Node is not ready. This corresponds to
the NodeCondition `Ready` being "`False`".
* `node.kubernetes.io/unreachable`: Node is unreachable from the node
controller. This corresponds to the NodeCondition `Ready` being "`Unknown`".
* `node.kubernetes.io/memory-pressure`: Node has memory pressure.
* `node.kubernetes.io/disk-pressure`: Node has disk pressure.
* `node.kubernetes.io/pid-pressure`: Node has PID pressure.
* `node.kubernetes.io/network-unavailable`: Node's network is unavailable.