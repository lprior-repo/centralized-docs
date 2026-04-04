---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#83-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 87
summary: * A ServiceAccount for CoreDNS is created in the `kube-system` namespace. * The `coredns` ServiceAccount is bound to the privileges in the `system:coredns` ClusterRole In Kubernetes version 1.21,...
---

* A ServiceAccount for CoreDNS is created in the `kube-system` namespace.
* The `coredns` ServiceAccount is bound to the privileges in the `system:coredns` ClusterRole
In Kubernetes version 1.21, support for using `kube-dns` with kubeadm was removed.
You can use CoreDNS with kubeadm even when the related Service is named `kube-dns`.