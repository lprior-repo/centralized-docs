---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#45-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 125
summary: on how to use `kubeadm kubeconfig user` to generate kubeconfig files for additional users. Make a record of the `kubeadm join` command that `kubeadm init` outputs. You need this command to [join...
---

on how to use `kubeadm kubeconfig user` to generate kubeconfig files for additional users.
Make a record of the `kubeadm join` command that `kubeadm init` outputs. You
need this command to [join nodes to your cluster](#join-nodes).
The token is used for mutual authentication between the control-plane node and the joining
nodes. The token included here is secret. Keep it safe, because anyone with this
token can add authenticated nodes to your cluster. These tokens can be listed,
created, and deleted with the `kubeadm token`