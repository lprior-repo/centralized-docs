---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#0-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 202
summary: ## Table of Contents  - [Configuring a cgroup driver](#configuring-a-cgroup-driver)   - [Before you begin](#before-you-begin)   - [Configuring the container runtime cgroup...
---

## Table of Contents

- [Configuring a cgroup driver](#configuring-a-cgroup-driver)
  - [Before you begin](#before-you-begin)
  - [Configuring the container runtime cgroup driver](#configuring-the-container-runtime-cgroup-driver)
  - [Configuring the kubelet cgroup driver](#configuring-the-kubelet-cgroup-driver)
      - [Note:](#note)
      - [Note:](#note)
  - [Using the `cgroupfs` driver](#using-the-cgroupfs-driver)
  - [Migrating to the `systemd` driver](#migrating-to-the-systemd-driver)
      - [Note:](#note)
    - [Modify the kubelet ConfigMap](#modify-the-kubelet-configmap)
    - [Update the cgroup driver on all nodes](#update-the-cgroup-driver-on-all-nodes)
  - [Feedback](#feedback)

---