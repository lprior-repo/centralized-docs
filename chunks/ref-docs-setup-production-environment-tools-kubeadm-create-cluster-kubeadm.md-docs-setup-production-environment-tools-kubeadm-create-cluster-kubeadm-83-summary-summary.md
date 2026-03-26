---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#83-summary
chunk_level: summary
chunk_type: prose
heading: Version skew policy
token_count: 118
summary: * Regularly [back up etcd](https://etcd.io/docs/v3.5/op-guide/recovery/). The etcd data directory configured by kubeadm is at `/var/lib/etcd` on the control-plane node. * Use multiple control-plane...
---

* Regularly [back up etcd](https://etcd.io/docs/v3.5/op-guide/recovery/). The
etcd data directory configured by kubeadm is at `/var/lib/etcd` on the control-plane node.
* Use multiple control-plane nodes. You can read
[Options for Highly Available topology](/docs/setup/production-environment/tools/kubeadm/ha-topology/) to pick a cluster
topology that provides [high-availability](/docs/setup/production-environment/tools/kubeadm/high-availability/).