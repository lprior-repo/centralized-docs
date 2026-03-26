---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#75-summary
chunk_level: summary
chunk_type: prose
heading: Version skew policy
token_count: 48
summary: ## Version skew policy While kubeadm allows version skew against some components that it manages, it is recommended that you match the kubeadm version with the versions of the control plane...
---

## Version skew policy
While kubeadm allows version skew against some components that it manages, it is recommended that you
match the kubeadm version with the versions of the control plane components, kube-proxy and kubelet.