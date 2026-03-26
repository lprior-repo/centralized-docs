---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#27-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 83
summary: ### Initializing your control-plane node The control-plane node is the machine where the control plane components run, including [etcd](/docs/tasks/administer-cluster/configure-upgrade-etcd/) (the...
---

### Initializing your control-plane node
The control-plane node is the machine where the control plane components run, including
[etcd](/docs/tasks/administer-cluster/configure-upgrade-etcd/) (the cluster database) and the
[API Server](/docs/concepts/architecture/#kube-apiserver)
(which the [kubectl](/docs/reference/kubectl/) command line tool
communicates with).