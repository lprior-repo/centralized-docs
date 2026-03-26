---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#50-summary
chunk_level: summary
chunk_type: prose
heading: External etcd nodes
token_count: 86
summary: * `ETCD\_1\_IP` * `ETCD\_2\_IP` The following steps are similar to the stacked etcd setup: 1. Run `sudo kubeadm init --config kubeadm-config.yaml --upload-certs` on this node. 2. Write the output...
---

* `ETCD\_1\_IP`
* `ETCD\_2\_IP`
The following steps are similar to the stacked etcd setup:
1. Run `sudo kubeadm init --config kubeadm-config.yaml --upload-certs` on this node.
2. Write the output join commands that are returned to a text file for later use.
3. Apply the CNI plugin of your choice.