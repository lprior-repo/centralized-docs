---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#102-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 119
summary: * Runs preflight checks similarly to `kubeadm init` and `kubeadm join`, ensuring container images are downloaded and the cluster is in a good state to be upgraded. * Upgrades the control plane...
---

* Runs preflight checks similarly to `kubeadm init` and `kubeadm join`, ensuring container images are downloaded
and the cluster is in a good state to be upgraded.
* Upgrades the control plane manifest files on disk in `/etc/kubernetes/manifests` and waits
for the kubelet to restart the components if the files have changed.
* Uploads the updated kubeadm and kubelet configurations to the cluster in the `kubeadm-config`
and the `kubelet-config` ConfigMaps (both in the `kube-system` namespace).