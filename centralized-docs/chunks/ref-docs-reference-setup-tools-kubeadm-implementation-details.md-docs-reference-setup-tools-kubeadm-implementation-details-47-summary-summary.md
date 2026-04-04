---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#47-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 94
summary: * The `address` that the controller-manager and the scheduler use to refer to the API server is `127.0.0.1` * If the etcd server is set up locally, the `etcd-server` address will be set to...
---

* The `address` that the controller-manager and the scheduler use to refer to the API server is `127.0.0.1`
* If the etcd server is set up locally, the `etcd-server` address will be set to `127.0.0.1:2379`
* Leader election is enabled for both the controller-manager and the scheduler
* Controller-manager and the scheduler will reference kubeconfig files with their respective, unique identities