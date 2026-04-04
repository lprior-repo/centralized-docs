---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#47-summary
chunk_level: summary
chunk_type: prose
heading: How it works
token_count: 109
summary: * Applies the new `CoreDNS` and `kube-proxy` manifests and makes sure that all necessary RBAC rules are created. * Creates new certificate and key files of the API server and backs up old files if...
---

* Applies the new `CoreDNS` and `kube-proxy` manifests and makes sure that all necessary RBAC rules are created.
* Creates new certificate and key files of the API server and backs up old files if they're about to expire in 180 days.
`kubeadm upgrade node` does the following on additional control plane nodes:
* Fetches the kubeadm `ClusterConfiguration` from the cluster.
* Optionally backups the kube-apiserver certificate.
* Upgrades the static Pod manifests for the control plane components.