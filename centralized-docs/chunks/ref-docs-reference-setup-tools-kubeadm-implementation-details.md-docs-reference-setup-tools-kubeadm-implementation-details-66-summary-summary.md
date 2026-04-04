---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#66-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 99
summary: ### Save the kubeadm ClusterConfiguration in a ConfigMap for later reference kubeadm saves the configuration passed to `kubeadm init` in a ConfigMap named `kubeadm-config` under `kube-system`...
---

### Save the kubeadm ClusterConfiguration in a ConfigMap for later reference
kubeadm saves the configuration passed to `kubeadm init` in a ConfigMap named `kubeadm-config`
under `kube-system` namespace.
This will ensure that kubeadm actions executed in future (e.g `kubeadm upgrade`) will be able to
determine the actual/current cluster state and make new decisions based on that data.
Please note that: