---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#80-summary
chunk_level: summary
chunk_type: prose
heading: Version skew policy
token_count: 116
summary: * kubeadm version 1.35 was used to create a cluster with `kubeadm init` * Joining nodes must use a kubeadm binary that is at version 1.35 Nodes that are being upgraded must use a version of kubeadm...
---

* kubeadm version 1.35 was used to create a cluster with `kubeadm init`
* Joining nodes must use a kubeadm binary that is at version 1.35
Nodes that are being upgraded must use a version of kubeadm that is the same MINOR
version or one MINOR version newer than the version of kubeadm used for managing the
node.
Example for `kubeadm upgrade`:
* kubeadm version 1.34 was used to create or upgrade the node