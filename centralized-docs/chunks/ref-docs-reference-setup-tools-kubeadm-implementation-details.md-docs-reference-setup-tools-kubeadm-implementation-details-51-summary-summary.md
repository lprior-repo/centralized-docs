---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#51-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 122
summary: * The `apiserver-advertise-address` and `apiserver-bind-port` to bind to; if not provided, those values default to the IP address of the default network interface on the machine and port 6443 * The...
---

* The `apiserver-advertise-address` and `apiserver-bind-port` to bind to; if not provided, those
values default to the IP address of the default network interface on the machine and port 6443
* The `service-cluster-ip-range` to use for services
* If an external etcd server is specified, the `etcd-servers` address and related TLS settings
(`etcd-cafile`, `etcd-certfile`, `etcd-keyfile`);
if an external etcd server is not provided, a local etcd will be used (via host network)