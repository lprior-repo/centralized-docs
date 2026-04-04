---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#34-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 91
summary: #### Note: On Linux nodes, the kubelet defaults to supporting only cgroups v2. For Kubernetes 1.35 the `FailCgroupV1` kubelet configuration option is set to `true` by default. To learn more, refer to...
---

#### Note:
On Linux nodes, the kubelet defaults to supporting only cgroups v2.
For Kubernetes 1.35 the `FailCgroupV1` kubelet configuration option is set to `true` by default.
To learn more, refer to the [Kubernetes cgroup v1 deprecation documentation](/docs/concepts/architecture/cgroups/#deprecation-of-cgroup-v1).
1. Upgrade the kubelet and kubectl: