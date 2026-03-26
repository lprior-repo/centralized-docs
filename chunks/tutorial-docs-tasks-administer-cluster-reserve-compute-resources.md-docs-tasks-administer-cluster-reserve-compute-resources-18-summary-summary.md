---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#18-summary
chunk_level: summary
chunk_type: prose
heading: Node Allocatable
token_count: 125
summary: * **KubeletConfiguration Setting**: `kubeReservedCgroup: \"\"` `kubeReserved` is meant to capture resource reservation for kubernetes system daemons like the `kubelet`, `container runtime`, etc. It is...
---

* **KubeletConfiguration Setting**: `kubeReservedCgroup: ""`
`kubeReserved` is meant to capture resource reservation for kubernetes system
daemons like the `kubelet`, `container runtime`, etc.
It is not meant to reserve resources for system daemons that are run as pods.
`kubeReserved` is typically a function of `pod density` on the nodes.
In addition to `cpu`, `memory`, and `ephemeral-storage`, `pid` may be
specified to reserve the specified number of process IDs for
kubernetes system daemons.
To optionally enforce `kubeReserved`