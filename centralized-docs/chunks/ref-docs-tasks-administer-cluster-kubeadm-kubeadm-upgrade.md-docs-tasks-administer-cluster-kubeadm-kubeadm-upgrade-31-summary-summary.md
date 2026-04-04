---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#31-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 109
summary: instances upgrade sequentially or at least ensure that the last control plane instance upgrade is not started until all the other control plane instances have been upgraded completely, and the addons...
---

instances upgrade sequentially or at least ensure that the last control plane instance upgrade is not started until all
the other control plane instances have been upgraded completely, and the addons upgrade will be performed after the last
control plane instance is upgraded.
5. Manually upgrade your CNI provider plugin.
Your Container Network Interface (CNI) provider may have its own upgrade instructions to follow.
Check the [addons](/docs/concepts/cluster-administration/addons/) page to
find your CNI provider and see whether additional upgrade steps are required.