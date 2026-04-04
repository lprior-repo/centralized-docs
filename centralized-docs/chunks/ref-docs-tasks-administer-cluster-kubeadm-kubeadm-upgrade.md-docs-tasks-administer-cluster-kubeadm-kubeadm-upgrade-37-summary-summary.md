---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#37-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 80
summary: 2. Restart the kubelet: ``` `sudo systemctl daemon-reload sudo systemctl restart kubelet ` ``` ### Uncordon the node Bring the node back online by marking it schedulable: ``` `# replace...
---

2. Restart the kubelet:
```
`sudo systemctl daemon-reload
sudo systemctl restart kubelet
`
```
### Uncordon the node
Bring the node back online by marking it schedulable:
```
`# replace &lt;node-to-uncordon&gt; with the name of your node
kubectl uncordon &lt;node-to-uncordon&gt;
`
```