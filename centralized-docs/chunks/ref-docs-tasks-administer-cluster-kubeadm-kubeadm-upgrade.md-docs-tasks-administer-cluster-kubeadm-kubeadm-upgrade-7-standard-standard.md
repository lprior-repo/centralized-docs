---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#7-standard
chunk_level: standard
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 305
summary: #### Note: For versions earlier than v1.28, kubeadm defaulted to a mode that upgrades the addons (including CoreDNS and kube-proxy) immediately during `kubeadm upgrade apply`, regardless of whether...
---

#### Note:
For versions earlier than v1.28, kubeadm defaulted to a mode that upgrades the addons (including CoreDNS and kube-proxy)
immediately during `kubeadm upgrade apply`, regardless of whether there are other control plane instances that have not
been upgraded. This may cause compatibility problems. Since v1.28, kubeadm defaults to a mode that checks whether all
the control plane instances have been upgraded before starting to upgrade the addons. You must perform control plane
instances upgrade sequentially or at least ensure that the last control plane instance upgrade is not started until all
the other control plane instances have been upgraded completely, and the addons upgrade will be performed after the last
control plane instance is upgraded.
5. Manually upgrade your CNI provider plugin.
Your Container Network Interface (CNI) provider may have its own upgrade instructions to follow.
Check the [addons](/docs/concepts/cluster-administration/addons/) page to
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