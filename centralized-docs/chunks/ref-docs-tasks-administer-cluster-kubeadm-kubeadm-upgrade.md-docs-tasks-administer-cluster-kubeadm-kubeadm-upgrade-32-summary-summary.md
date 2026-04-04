---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#32-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 109
summary: page to find your CNI provider and see whether additional upgrade steps are required. This step is not required on additional control plane nodes if the CNI provider runs as a DaemonSet. **For the...
---

 page to
find your CNI provider and see whether additional upgrade steps are required.
This step is not required on additional control plane nodes if the CNI provider runs as a DaemonSet.
**For the other control plane nodes**
Same as the first control plane node but use:
```
`sudo kubeadm upgrade node
`
```
instead of:
```
`sudo kubeadm upgrade apply
`
```
Also calling `kubeadm upgrade plan` and upgrading the CNI provider plugin is no longer needed.