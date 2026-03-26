---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#19-summary
chunk_level: summary
chunk_type: prose
heading: Node Allocatable
token_count: 128
summary: `pid` may be specified to reserve the specified number of process IDs for kubernetes system daemons. To optionally enforce `kubeReserved` on kubernetes system daemons, specify the parent control...
---

`pid` may be
specified to reserve the specified number of process IDs for
kubernetes system daemons.
To optionally enforce `kubeReserved` on kubernetes system daemons, specify the parent
control group for kube daemons as the value for `kubeReservedCgroup` setting,
and [add `kube-reserved` to `enforceNodeAllocatable`](#enforcing-node-allocatable).
It is recommended that the kubernetes system daemons are placed under a top
level control group (`runtime.slice` on systemd machines for example). Each
system daemon should ideally run within its own child control group. Refer to