---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#72-summary
chunk_level: summary
chunk_type: prose
heading: Clean up
token_count: 106
summary: ### Remove the node Talking to the control-plane node with the appropriate credentials, run: ``` `kubectl drain &lt;node name&gt; --delete-emptydir-data --force --ignore-daemonsets ` ``` Before...
---

### Remove the node
Talking to the control-plane node with the appropriate credentials, run:
```
`kubectl drain &lt;node name&gt; --delete-emptydir-data --force --ignore-daemonsets
`
```
Before removing the node, reset the state installed by `kubeadm`:
```
`kubeadm reset
`
```
The reset process does not reset or clean up iptables rules or IPVS tables.
If you wish to reset iptables, you must do so manually: