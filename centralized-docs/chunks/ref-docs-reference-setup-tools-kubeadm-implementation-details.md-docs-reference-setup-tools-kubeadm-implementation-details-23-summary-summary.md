---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#23-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 121
summary: * [Error] if user is not root * [Error] if the machine hostname is not a valid DNS subdomain * [Warning] if the host name cannot be reached via network lookup * [Error] if kubelet version is lower...
---

* [Error] if user is not root
* [Error] if the machine hostname is not a valid DNS subdomain
* [Warning] if the host name cannot be reached via network lookup
* [Error] if kubelet version is lower that the minimum kubelet version supported by kubeadm (current minor -1)
* [Error] if kubelet version is at least one minor higher than the required controlplane version (unsupported version skew)
* [Warning] if kubelet service does not exist or if it is disabled
* [Warning] if firewalld is active