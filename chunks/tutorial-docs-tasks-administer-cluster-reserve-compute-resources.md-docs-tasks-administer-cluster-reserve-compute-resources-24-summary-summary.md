---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#24-summary
chunk_level: summary
chunk_type: prose
heading: Node Allocatable
token_count: 128
summary: * **KubeletConfiguration Setting**: `systemReservedCgroup: \"\"` `systemReserved` is meant to capture resource reservation for OS system daemons like `sshd`, `udev`, etc. `systemReserved` should...
---

* **KubeletConfiguration Setting**: `systemReservedCgroup: ""`
`systemReserved` is meant to capture resource reservation for OS system daemons
like `sshd`, `udev`, etc. `systemReserved` should reserve `memory` for the
`kernel` too since `kernel` memory is not accounted to pods in Kubernetes at this time.
Reserving resources for user login sessions is also recommended (`user.slice` in
systemd world).
In addition to `cpu`, `memory`, and `ephemeral-storage`, `pid` may be
specified to reserve the specified number of process IDs for OS system