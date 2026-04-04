---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#18-summary
chunk_level: summary
chunk_type: prose
heading: Changing the package repository
token_count: 117
summary: If you're using the community-owned package repositories (`pkgs.k8s.io`), you need to enable the package repository for the desired Kubernetes minor release. This is explained in [Changing the...
---

If you're using the community-owned package repositories (`pkgs.k8s.io`), you need to
enable the package repository for the desired Kubernetes minor release. This is explained in
[Changing the Kubernetes package repository](/docs/tasks/administer-cluster/kubeadm/change-package-repository/)
document.
**Note:** The legacy package repositories (`apt.kubernetes.io` and `yum.kubernetes.io`) have been
[deprecated and frozen starting from September 13, 2023](/blog/2023/08/31/legacy-package-repository-deprecation/).