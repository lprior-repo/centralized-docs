---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#42-summary
chunk_level: summary
chunk_type: prose
heading: External etcd nodes
token_count: 82
summary: ### Set up the etcd cluster 1. Follow these [instructions](/docs/setup/production-environment/tools/kubeadm/setup-ha-etcd-with-kubeadm/) to set up the etcd cluster. 2. Set up SSH as described...
---

### Set up the etcd cluster
1. Follow these [instructions](/docs/setup/production-environment/tools/kubeadm/setup-ha-etcd-with-kubeadm/) to set up the etcd cluster.
2. Set up SSH as described [here](#manual-certs).
3. Copy the following files from any etcd node in the cluster to the first control plane node: