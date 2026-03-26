---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#6-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the container runtime cgroup driver
token_count: 124
summary: The [Container runtimes](/docs/setup/production-environment/container-runtimes/) page explains that the `systemd` driver is recommended for kubeadm based setups instead of the kubelet's...
---

The [Container runtimes](/docs/setup/production-environment/container-runtimes/) page
explains that the `systemd` driver is recommended for kubeadm based setups instead
of the kubelet's [default](/docs/reference/config-api/kubelet-config.v1beta1/) `cgroupfs` driver,
because kubeadm manages the kubelet as a
[systemd service](/docs/setup/production-environment/tools/kubeadm/kubelet-integration/).
The page also provides details on how to set up a number of different container runtimes with the