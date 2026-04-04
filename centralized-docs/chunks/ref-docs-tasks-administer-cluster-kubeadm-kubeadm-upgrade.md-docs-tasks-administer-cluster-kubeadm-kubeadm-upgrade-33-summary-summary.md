---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#33-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 68
summary: ### Drain the node Prepare the node for maintenance by marking it unschedulable and evicting the workloads: ``` `# replace &lt;node-to-drain&gt; with the name of your node you are draining kubectl...
---

### Drain the node
Prepare the node for maintenance by marking it unschedulable and evicting the workloads:
```
`# replace &lt;node-to-drain&gt; with the name of your node you are draining
kubectl drain &lt;node-to-drain&gt; --ignore-daemonsets
`
```