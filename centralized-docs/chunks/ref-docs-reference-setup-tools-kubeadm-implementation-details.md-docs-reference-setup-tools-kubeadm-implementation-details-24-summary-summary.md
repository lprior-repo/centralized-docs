---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#24-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 106
summary: * [Warning] if kubelet service does not exist or if it is disabled * [Warning] if firewalld is active * [Error] if API server bindPort or ports 10250/10251/10252 are used * [Error] if...
---

* [Warning] if kubelet service does not exist or if it is disabled
* [Warning] if firewalld is active
* [Error] if API server bindPort or ports 10250/10251/10252 are used
* [Error] if `/etc/kubernetes/manifest` folder already exists and it is not empty
* [Error] if swap is on
* [Error] if `ip`, `iptables`, `mount`, `nsenter` commands are not present in the command path