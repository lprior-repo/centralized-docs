---
doc_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources
chunk_id: tutorial/docs-tasks-administer-cluster-reserve-compute-resources.md/docs-tasks-administer-cluster-reserve-compute-resources#15-summary
chunk_level: summary
chunk_type: prose
heading: Node Allocatable
token_count: 100
summary: * `systemd` is an alternative driver that manages cgroup sandboxes using transient slices for resources that are supported by that init system. Depending on the configuration of the associated...
---

* `systemd` is an alternative driver that manages cgroup sandboxes using
transient slices for resources that are supported by that init system.
Depending on the configuration of the associated container runtime,
operators may have to choose a particular cgroup driver to ensure
proper system behavior. For example, if operators use the `systemd`
cgroup driver provided by the `containerd` runtime, the `kubelet` must
be configured to use the `systemd` cgroup driver.