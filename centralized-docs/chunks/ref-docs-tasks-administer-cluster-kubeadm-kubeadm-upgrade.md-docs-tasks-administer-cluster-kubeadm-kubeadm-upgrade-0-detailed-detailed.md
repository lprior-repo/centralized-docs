---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 341
summary: ## Table of Contents  - [Upgrading kubeadm clusters](#upgrading-kubeadm-clusters)   - [Changing the package repository](#changing-the-package-repository)   - [Determine which version to upgrade...
---

## Table of Contents

- [Upgrading kubeadm clusters](#upgrading-kubeadm-clusters)
  - [Changing the package repository](#changing-the-package-repository)
  - [Determine which version to upgrade to](#determine-which-version-to-upgrade-to)
- [It should look like 1.35.x-\*, where x is the latest patch.](#it-should-look-like-135x--where-x-is-the-latest-patch)
- [It should look like 1.35.x-\*, where x is the latest patch.](#it-should-look-like-135x--where-x-is-the-latest-patch)
- [It should look like 1.35.x-\*, where x is the latest patch.](#it-should-look-like-135x--where-x-is-the-latest-patch)
  - [Upgrading control plane nodes](#upgrading-control-plane-nodes)
    - [Call "kubeadm upgrade"](#call-kubeadm-upgrade)
      - [Note:](#note)
      - [Note:](#note)
    - [Drain the node](#drain-the-node)
      - [Note:](#note)
    - [Uncordon the node](#uncordon-the-node)
  - [Upgrade worker nodes](#upgrade-worker-nodes)
  - [Recovering from a failure state](#recovering-from-a-failure-state)
      - [Note:](#note)
  - [How it works](#how-it-works)
  - [Feedback](#feedback)

---