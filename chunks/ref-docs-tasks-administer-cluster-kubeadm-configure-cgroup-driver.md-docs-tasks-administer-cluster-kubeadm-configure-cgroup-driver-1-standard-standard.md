---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#1-standard
chunk_level: standard
chunk_type: prose
heading: Configuring the container runtime cgroup driver
token_count: 211
summary: # Configuring a cgroup driver This page explains how to configure the kubelet's cgroup driver to match the container runtime cgroup driver for kubeadm clusters. ## Before you begin You should be...
---

# Configuring a cgroup driver
This page explains how to configure the kubelet's cgroup driver to match the container
runtime cgroup driver for kubeadm clusters.
## Before you begin
You should be familiar with the Kubernetes
[container runtime requirements](/docs/setup/production-environment/container-runtimes/).
## Configuring the container runtime cgroup driver
The [Container runtimes](/docs/setup/production-environment/container-runtimes/) page
explains that the `systemd` driver is recommended for kubeadm based setups instead
of the kubelet's [default](/docs/reference/config-api/kubelet-config.v1beta1/) `cgroupfs` driver,
because kubeadm manages the kubelet as a
[systemd service](/docs/setup/production-environment/tools/kubeadm/kubelet-integration/).
The page also provides details on how to set up a number of different container runtimes with the
`systemd` driver by default.