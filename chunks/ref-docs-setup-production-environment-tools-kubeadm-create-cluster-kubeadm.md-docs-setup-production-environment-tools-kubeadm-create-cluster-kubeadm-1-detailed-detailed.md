---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Before you begin
token_count: 530
summary: # Creating a cluster with kubeadm ![](/images/kubeadm-stacked-color.png) Using `kubeadm`, you can create a minimum viable Kubernetes cluster that conforms to best practices. In fact, you can use...
---

# Creating a cluster with kubeadm
![](/images/kubeadm-stacked-color.png)
Using `kubeadm`, you can create a minimum viable Kubernetes cluster that conforms to best practices.
In fact, you can use `kubeadm` to set up a cluster that will pass the
[Kubernetes Conformance tests](/blog/2017/10/software-conformance-certification/).
`kubeadm` also supports other cluster lifecycle functions, such as
[bootstrap tokens](/docs/reference/access-authn-authz/bootstrap-tokens/) and cluster upgrades.
The `kubeadm` tool is good if you need:
* A simple way for you to try out Kubernetes, possibly for the first time.
* A way for existing users to automate setting up a cluster and test their application.
* A building block in other ecosystem and/or installer tools with a larger
scope.
You can install and use `kubeadm` on various machines: your laptop, a set
of cloud servers, a Raspberry Pi, and more. Whether you're deploying into the
cloud or on-premises, you can integrate `kubeadm` into provisioning systems such
as Ansible or Terraform.
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