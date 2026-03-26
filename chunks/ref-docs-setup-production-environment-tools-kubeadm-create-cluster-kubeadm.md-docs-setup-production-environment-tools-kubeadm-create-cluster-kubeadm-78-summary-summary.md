---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#78-summary
chunk_level: summary
chunk_type: prose
heading: Version skew policy
token_count: 124
summary: * kubelet on the host must be at 1.35, 1.34, 1.33 or 1.32### kubeadm's skew against kubeadm There are certain limitations on how kubeadm commands can operate on existing nodes or whole clusters...
---

* kubelet on the host must be at 1.35, 1.34,
1.33 or 1.32### kubeadm's skew against kubeadm
There are certain limitations on how kubeadm commands can operate on existing nodes or whole clusters
managed by kubeadm.
If new nodes are joined to the cluster, the kubeadm binary used for `kubeadm join` must match
the last version of kubeadm used to either create the cluster with `kubeadm init` or to upgrade
the same node with