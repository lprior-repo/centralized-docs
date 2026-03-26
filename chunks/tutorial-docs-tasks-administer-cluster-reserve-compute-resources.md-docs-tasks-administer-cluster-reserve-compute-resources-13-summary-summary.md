---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#13-summary
chunk_level: summary
chunk_type: prose
heading: Node Allocatable
token_count: 50
summary: ### Configuring a cgroup driver The `kubelet` supports manipulation of the cgroup hierarchy on the host using a cgroup driver. The driver is configured via the `cgroupDriver` setting. The supported...
---

### Configuring a cgroup driver
The `kubelet` supports manipulation of the cgroup hierarchy on
the host using a cgroup driver. The driver is configured via the `cgroupDriver` setting.
The supported values are the following: