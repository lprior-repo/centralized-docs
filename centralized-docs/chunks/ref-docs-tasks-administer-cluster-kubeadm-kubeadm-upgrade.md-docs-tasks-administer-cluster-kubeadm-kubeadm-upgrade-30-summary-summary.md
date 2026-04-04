---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#30-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading control plane nodes
token_count: 120
summary: For versions earlier than v1.28, kubeadm defaulted to a mode that upgrades the addons (including CoreDNS and kube-proxy) immediately during `kubeadm upgrade apply`, regardless of whether there are...
---

For versions earlier than v1.28, kubeadm defaulted to a mode that upgrades the addons (including CoreDNS and kube-proxy)
immediately during `kubeadm upgrade apply`, regardless of whether there are other control plane instances that have not
been upgraded. This may cause compatibility problems. Since v1.28, kubeadm defaults to a mode that checks whether all
the control plane instances have been upgraded before starting to upgrade the addons. You must perform control plane
instances upgrade sequentially or at least ensure that the last control plane instance upgrade is not started until all