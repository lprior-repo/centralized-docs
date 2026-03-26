---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#2-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 276
summary: ## Before you begin To follow this guide, you need: * One or more machines running a deb/rpm-compatible Linux OS; for example: Ubuntu or CentOS. * 2 GiB or more of RAM per machine--any less leaves...
---

## Before you begin
To follow this guide, you need:
* One or more machines running a deb/rpm-compatible Linux OS; for example: Ubuntu or CentOS.
* 2 GiB or more of RAM per machine--any less leaves little room for your apps.
* At least 2 CPUs on the machine that you use as a control-plane node.
* Full network connectivity among all machines in the cluster. You can use either a
public or a private network.
You also need to use a version of `kubeadm` that can deploy the version
of Kubernetes that you want to use in your new cluster.
[Kubernetes' version and version skew support policy](/docs/setup/release/version-skew-policy/#supported-versions)
applies to `kubeadm` as well as to Kubernetes overall.
Check that policy to learn about what versions of Kubernetes and `kubeadm`
are supported. This page is written for Kubernetes v1.35.
The `kubeadm` tool's overall feature state is General Availability (GA). Some sub-features are
still under active development. The implementation of creating the cluster may change
slightly as the tool evolves, but the overall implementation should be pretty stable.
#### Note:
Any commands under `kubeadm alpha` are, by definition, supported on an alpha level.