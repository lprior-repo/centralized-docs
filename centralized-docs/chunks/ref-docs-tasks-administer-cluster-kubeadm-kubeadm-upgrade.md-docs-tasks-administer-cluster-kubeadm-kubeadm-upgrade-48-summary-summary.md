---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#48-summary
chunk_level: summary
chunk_type: prose
heading: How it works
token_count: 76
summary: * Optionally backups the kube-apiserver certificate. * Upgrades the static Pod manifests for the control plane components. * Upgrades the kubelet configuration for this node. `kubeadm upgrade node`...
---

* Optionally backups the kube-apiserver certificate.
* Upgrades the static Pod manifests for the control plane components.
* Upgrades the kubelet configuration for this node.
`kubeadm upgrade node` does the following on worker nodes:
* Fetches the kubeadm `ClusterConfiguration` from the cluster.
* Upgrades the kubelet configuration for this node.