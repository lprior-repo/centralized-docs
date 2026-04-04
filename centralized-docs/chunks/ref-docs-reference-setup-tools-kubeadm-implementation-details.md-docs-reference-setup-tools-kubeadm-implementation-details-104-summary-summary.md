---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#104-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 32
summary: * Upgrades the kube-proxy and CoreDNS addons conditionally if all existing kube-apiservers in the cluster have already been upgraded to the target version.
---

* Upgrades the kube-proxy and CoreDNS addons conditionally if all existing kube-apiservers in the cluster
have already been upgraded to the target version.