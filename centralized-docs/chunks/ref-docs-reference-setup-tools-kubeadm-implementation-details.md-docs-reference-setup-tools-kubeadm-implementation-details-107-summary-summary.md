---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#107-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 51
summary: * (For control plane nodes) upgrades the kube-proxy and CoreDNS [addons](/docs/concepts/cluster-administration/addons/) conditionally, provided that all existing API servers in the cluster have...
---

* (For control plane nodes) upgrades the kube-proxy and CoreDNS
[addons](/docs/concepts/cluster-administration/addons/) conditionally, provided that all existing
API servers in the cluster have already been upgraded to the target version.