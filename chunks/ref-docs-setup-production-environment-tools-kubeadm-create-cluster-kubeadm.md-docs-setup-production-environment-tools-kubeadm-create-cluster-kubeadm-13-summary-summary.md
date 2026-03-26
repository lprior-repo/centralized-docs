---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#13-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 126
summary: * Full network connectivity among all machines in the cluster. You can use either a public or a private network. You also need to use a version of `kubeadm` that can deploy the version of Kubernetes...
---

* Full network connectivity among all machines in the cluster. You can use either a
public or a private network.
You also need to use a version of `kubeadm` that can deploy the version
of Kubernetes that you want to use in your new cluster.
[Kubernetes' version and version skew support policy](/docs/setup/release/version-skew-policy/#supported-versions)
applies to `kubeadm` as well as to Kubernetes overall.
Check that policy to learn about what versions of Kubernetes and `kubeadm`
are supported. This page is written for Kubernetes v1.35.
The