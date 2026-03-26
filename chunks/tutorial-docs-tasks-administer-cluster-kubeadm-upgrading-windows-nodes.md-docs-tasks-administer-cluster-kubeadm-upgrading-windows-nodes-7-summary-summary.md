---
doc_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes
chunk_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes#7-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 119
summary: ### Drain the node 1. From a machine with access to the Kubernetes API, prepare the node for maintenance by marking it unschedulable and evicting the workloads: ``` `# replace &lt;node-to-drain&gt;...
---

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