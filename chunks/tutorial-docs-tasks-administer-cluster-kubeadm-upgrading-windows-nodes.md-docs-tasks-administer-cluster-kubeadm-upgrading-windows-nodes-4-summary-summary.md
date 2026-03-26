---
doc_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes
chunk_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes#4-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 83
summary: ## Before you begin You need to have shell access to all the nodes, and the kubectl command-line tool must be configured to communicate with your cluster. It is recommended to run this tutorial on a...
---

## Before you begin
You need to have shell access to all the nodes, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial
on a cluster with at least two nodes that are not acting as control plane hosts.
Your Kubernetes server must be at or later than version 1.17.
To check the version, enter `kubectl version`.