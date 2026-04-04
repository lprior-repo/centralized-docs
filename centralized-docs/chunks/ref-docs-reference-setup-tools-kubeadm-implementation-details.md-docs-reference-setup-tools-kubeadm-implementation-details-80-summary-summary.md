---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#80-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 67
summary: ### Install addons Kubeadm installs the internal DNS server and the kube-proxy addon components via the API server. #### Note: This phase can be invoked individually with the command [`kubeadm init...
---

### Install addons
Kubeadm installs the internal DNS server and the kube-proxy addon components via the API server.
#### Note:
This phase can be invoked individually with the command
[`kubeadm init phase addon all`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-addon).