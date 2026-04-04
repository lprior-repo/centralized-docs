---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 377
summary: # Upgrading kubeadm clusters This page explains how to upgrade a Kubernetes cluster created with kubeadm from version 1.34.x to version 1.35.x, and from version 1.35.x to 1.35.y (where `y &gt; x`)....
---

# Upgrading kubeadm clusters
This page explains how to upgrade a Kubernetes cluster created with kubeadm from version
1.34.x to version 1.35.x, and from version
1.35.x to 1.35.y (where `y &gt; x`). Skipping MINOR versions
when upgrading is unsupported. For more details, please visit [Version Skew Policy](/releases/version-skew-policy/).
To see information about upgrading clusters created using older versions of kubeadm,
please refer to following pages instead:
* [Upgrading a kubeadm cluster from 1.33 to 1.34](https://v1-34.docs.kubernetes.io/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/)
* [Upgrading a kubeadm cluster from 1.32 to 1.33](https://v1-33.docs.kubernetes.io/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/)
* [Upgrading a kubeadm cluster from 1.31 to 1.32](https://v1-32.docs.kubernetes.io/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/)
* [Upgrading a kubeadm cluster from 1.30 to 1.31](https://v1-31.docs.kubernetes.io/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/)
The Kubernetes project recommends upgrading to the latest patch releases promptly, and
to ensure that you are running a supported minor release of Kubernetes.
Following this recommendation helps you to stay secure.
The upgrade workflow at high level is the following:
1. Upgrade a primary control plane node.
2. Upgrade additional control plane nodes.
3. Upgrade worker nodes.## Before you begin