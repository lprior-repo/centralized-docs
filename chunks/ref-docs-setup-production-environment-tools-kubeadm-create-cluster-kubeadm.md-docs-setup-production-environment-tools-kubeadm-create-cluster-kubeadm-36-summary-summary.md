---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#36-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 128
summary: for control plane components and etcd server, provide extra arguments to each component as documented in [custom arguments](/docs/setup/production-environment/tools/kubeadm/control-plane-flags/). To...
---

for control plane components and etcd server, provide extra arguments to each component as documented in
[custom arguments](/docs/setup/production-environment/tools/kubeadm/control-plane-flags/).
To reconfigure a cluster that has already been created see
[Reconfiguring a kubeadm cluster](/docs/tasks/administer-cluster/kubeadm/kubeadm-reconfigure/).
To run `kubeadm init` again, you must first [tear down the cluster](#tear-down).
If you join a node with a different architecture to your cluster, make sure that your deployed DaemonSets