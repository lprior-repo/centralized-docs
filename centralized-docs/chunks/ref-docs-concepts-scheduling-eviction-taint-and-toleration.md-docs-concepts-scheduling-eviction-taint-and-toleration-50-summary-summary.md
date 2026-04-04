---
doc_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration
chunk_id: ref/docs-concepts-scheduling-eviction-taint-and-toleration.md/docs-concepts-scheduling-eviction-taint-and-toleration#50-summary
chunk_level: summary
chunk_type: prose
heading: Taint based Evictions
token_count: 115
summary: * `node.kubernetes.io/unreachable` * `node.kubernetes.io/not-ready` This ensures that DaemonSet pods are never evicted due to these problems. #### Note: The node controller was responsible for adding...
---

* `node.kubernetes.io/unreachable`
* `node.kubernetes.io/not-ready`
This ensures that DaemonSet pods are never evicted due to these problems.
#### Note:
The node controller was responsible for adding taints to nodes and evicting pods. But after 1.29,
the taint-based eviction implementation has been moved out of node controller into a separate,
and independent component called taint-eviction-controller. Users can optionally disable taint-based
eviction by setting `--controllers=-taint-eviction-controller` in kube-controller-manager.