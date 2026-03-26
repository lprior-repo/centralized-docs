---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#77-summary
chunk_level: summary
chunk_type: prose
heading: Version skew policy
token_count: 89
summary: * kubeadm is at 1.35 * `kubernetesVersion` must be at 1.35 or 1.34### kubeadm's skew against the kubelet Similarly to the Kubernetes version, kubeadm can be used with a kubelet version that is the...
---

* kubeadm is at 1.35
* `kubernetesVersion` must be at 1.35 or 1.34### kubeadm's skew against the kubelet
Similarly to the Kubernetes version, kubeadm can be used with a kubelet version that is
the same version as kubeadm or three versions older.
Example:
* kubeadm is at 1.35