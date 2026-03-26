---
doc_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes
chunk_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes#2-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 382
summary: ## Before you begin You need to have shell access to all the nodes, and the kubectl command-line tool must be configured to communicate with your cluster. It is recommended to run this tutorial on a...
---

## Before you begin
You need to have shell access to all the nodes, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial
on a cluster with at least two nodes that are not acting as control plane hosts.
Your Kubernetes server must be at or later than version 1.17.
To check the version, enter `kubectl version`.
* Familiarize yourself with [the process for upgrading the rest of your kubeadm
cluster](/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/). You will want to
upgrade the control plane nodes before upgrading your Windows nodes.## Upgrading worker nodes
### Upgrade kubeadm
1. From the Windows node, upgrade kubeadm:
```
`# replace 1.35.0 with your desired version
curl.exe -Lo &lt;path-to-kubeadm.exe&gt; "https://dl.k8s.io/v1.35.0/bin/windows/amd64/kubeadm.exe"
`
```
### Drain the node
1. From a machine with access to the Kubernetes API,
prepare the node for maintenance by marking it unschedulable and evicting the workloads:
```
`# replace &lt;node-to-drain&gt; with the name of your node you are draining
kubectl drain &lt;node-to-drain&gt; --ignore-daemonsets
`
```
You should see output similar to this:
```
`node/ip-172-31-85-18 cordoned
node/ip-172-31-85-18 drained
`
```
### Upgrade the kubelet configuration
1. From the Windows node, call the following command to sync new kubelet configuration:
```
`kubeadm upgrade node
`
```