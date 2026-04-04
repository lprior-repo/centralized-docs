---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#25-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 112
summary: * [Error] if `ip`, `iptables`, `mount`, `nsenter` commands are not present in the command path * [Warning] if `ethtool`, `tc`, `touch` commands are not present in the command path * [Warning] if...
---

* [Error] if `ip`, `iptables`, `mount`, `nsenter` commands are not present in the command path
* [Warning] if `ethtool`, `tc`, `touch` commands are not present in the command path
* [Warning] if extra arg flags for API server, controller manager, scheduler contains some invalid options
* [Warning] if connection to https://API.AdvertiseAddress:API.BindPort goes through proxy
* [Warning] if connection to services subnet goes through proxy (only first address checked)