---
doc_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes
chunk_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes#11-summary
chunk_level: summary
chunk_type: prose
heading: What's next
token_count: 97
summary: ### Uncordon the node 1. From a machine with access to the Kubernetes API, bring the node back online by marking it schedulable: ``` `# replace &lt;node-to-drain&gt; with the name of your node...
---

### Uncordon the node
1. From a machine with access to the Kubernetes API,
bring the node back online by marking it schedulable:
```
`# replace &lt;node-to-drain&gt; with the name of your node
kubectl uncordon &lt;node-to-drain&gt;
`
```
## What's next
* See how to [Upgrade Linux nodes](/docs/tasks/administer-cluster/kubeadm/upgrading-linux-nodes/).