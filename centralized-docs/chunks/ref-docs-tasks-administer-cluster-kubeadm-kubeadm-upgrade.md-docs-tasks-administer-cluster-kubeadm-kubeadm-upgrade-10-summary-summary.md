---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#10-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 119
summary: * Make sure you read the [release notes](https://git.k8s.io/kubernetes/CHANGELOG) carefully. * The cluster should use a static control plane and etcd pods or external etcd. * Make sure to back up any...
---

* Make sure you read the [release notes](https://git.k8s.io/kubernetes/CHANGELOG) carefully.
* The cluster should use a static control plane and etcd pods or external etcd.
* Make sure to back up any important components, such as app-level state stored in a database.
`kubeadm upgrade` does not touch your workloads, only components internal to Kubernetes, but backups are always a best practice.
* [Swap must be disabled](https://serverfault.com/questions/684771/best-way-to-disable-swap-in-linux).### Additional information