---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 254
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