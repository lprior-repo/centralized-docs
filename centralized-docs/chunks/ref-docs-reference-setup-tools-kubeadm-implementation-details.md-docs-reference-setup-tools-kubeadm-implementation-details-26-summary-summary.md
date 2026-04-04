---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#26-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 124
summary: * [Warning] if connection to services subnet goes through proxy (only first address checked) * [Warning] if connection to Pods subnet goes through proxy (only first address checked) * If external...
---

* [Warning] if connection to services subnet goes through proxy (only first address checked)
* [Warning] if connection to Pods subnet goes through proxy (only first address checked)
* If external etcd is provided:
* [Error] if etcd version is older than the minimum required version
* [Error] if etcd certificates or keys are specified, but not provided
* If external etcd is NOT provided (and thus local etcd will be installed):
* [Error] if ports 2379 is used
* [Error] if Etcd.DataDir folder already exists and it is not empty