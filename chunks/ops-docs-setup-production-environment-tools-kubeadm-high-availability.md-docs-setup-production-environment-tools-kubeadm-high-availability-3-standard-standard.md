---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#3-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 494
summary: * Three or more machines that meet [kubeadm's minimum requirements](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#before-you-begin) for the control-plane nodes. Having an odd...
---

* Three or more machines that meet [kubeadm's minimum requirements](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#before-you-begin) for
the control-plane nodes. Having an odd number of control plane nodes can help
with leader selection in the case of machine or zone failure.
* including a [container runtime](/docs/setup/production-environment/container-runtimes), already set up and working
* Three or more machines that meet [kubeadm's minimum
requirements](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#before-you-begin) for the workers
* including a container runtime, already set up and working
* Full network connectivity between all machines in the cluster (public or
private network)
* Superuser privileges on all machines using `sudo`
* You can use a different tool; this guide uses `sudo` in the examples.
* SSH access from one device to all nodes in the system
* `kubeadm` and `kubelet` already installed on all machines.
*See [Stacked etcd topology](/docs/setup/production-environment/tools/kubeadm/ha-topology/#stacked-etcd-topology) for context.*
You need:
* Three or more machines that meet [kubeadm's minimum requirements](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#before-you-begin) for
the control-plane nodes. Having an odd number of control plane nodes can help
with leader selection in the case of machine or zone failure.
* including a [container runtime](/docs/setup/production-environment/container-runtimes), already set up and working
* Three or more machines that meet [kubeadm's minimum
requirements](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#before-you-begin) for the workers
* including a container runtime, already set up and working
* Full network connectivity between all machines in the cluster (public or
private network)
* Superuser privileges on all machines using `sudo`
* You can use a different tool; this guide uses `sudo` in the examples.
* SSH access from one device to all nodes in the system
* `kubeadm` and `kubelet` already installed on all machines.
And you also need: