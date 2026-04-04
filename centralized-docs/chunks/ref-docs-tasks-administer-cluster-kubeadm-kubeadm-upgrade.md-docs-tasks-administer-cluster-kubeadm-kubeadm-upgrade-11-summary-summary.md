---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#11-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 122
summary: * [Swap must be disabled](https://serverfault.com/questions/684771/best-way-to-disable-swap-in-linux).### Additional information * The instructions below outline when to drain each node during the...
---

* [Swap must be disabled](https://serverfault.com/questions/684771/best-way-to-disable-swap-in-linux).### Additional information
* The instructions below outline when to drain each node during the upgrade process.
If you are performing a **minor** version upgrade for any kubelet, you **must**
first drain the node (or nodes) that you are upgrading. In the case of control plane nodes,
they could be running CoreDNS Pods or other critical workloads. For more information see
[Draining nodes](/docs/tasks/administer-cluster/safely-drain-node/).