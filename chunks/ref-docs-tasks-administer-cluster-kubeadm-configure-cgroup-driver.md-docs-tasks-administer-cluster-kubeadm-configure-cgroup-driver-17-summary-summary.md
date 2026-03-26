---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#17-summary
chunk_level: summary
chunk_type: prose
heading: Migrating to the `systemd` driver
token_count: 122
summary: ## Migrating to the `systemd` driver To change the cgroup driver of an existing kubeadm cluster from `cgroupfs` to `systemd` in-place, a similar procedure to a kubelet upgrade is required. This must...
---

## Migrating to the `systemd` driver
To change the cgroup driver of an existing kubeadm cluster from `cgroupfs` to `systemd` in-place,
a similar procedure to a kubelet upgrade is required. This must include both
steps outlined below.
#### Note:
Alternatively, it is possible to replace the old nodes in the cluster with new ones
that use the `systemd` driver. This requires executing only the first step below
before joining the new nodes and ensuring the workloads can safely move to the new
nodes before deleting the old nodes.