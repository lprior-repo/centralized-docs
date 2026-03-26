---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#37-summary
chunk_level: summary
chunk_type: prose
heading: Node Allocatable
token_count: 127
summary: on the [node pressure eviction](/docs/concepts/scheduling-eviction/node-pressure-eviction/) page. This enforcement is controlled by specifying `pods` value to the KubeletConfiguration setting...
---

on the [node pressure eviction](/docs/concepts/scheduling-eviction/node-pressure-eviction/)
page. This enforcement is controlled by
specifying `pods` value to the KubeletConfiguration setting `enforceNodeAllocatable`.
Optionally, `kubelet` can be made to enforce `kubeReserved` and
`systemReserved` by specifying `kube-reserved` &amp; `system-reserved` values in
the same setting. Additionally, only compressible resources may be enforced by
specifying `kube-reserved-compressible` and `system-reserved-compressible`.
Note that to enforce