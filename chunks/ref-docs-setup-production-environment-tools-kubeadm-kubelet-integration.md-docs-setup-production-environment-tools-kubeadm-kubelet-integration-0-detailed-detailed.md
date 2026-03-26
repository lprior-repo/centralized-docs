---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 423
summary: ## Table of Contents  - [Configuring each kubelet in your cluster using kubeadm](#configuring-each-kubelet-in-your-cluster-using-kubeadm)   - [Kubelet configuration...
---

## Table of Contents

- [Configuring each kubelet in your cluster using kubeadm](#configuring-each-kubelet-in-your-cluster-using-kubeadm)
  - [Kubelet configuration patterns](#kubelet-configuration-patterns)
    - [Propagating cluster-level configuration to each kubelet](#propagating-cluster-level-configuration-to-each-kubelet)
    - [Providing instance-specific configuration details](#providing-instance-specific-configuration-details)
  - [Configure kubelets using kubeadm](#configure-kubelets-using-kubeadm)
    - [Workflow when using `kubeadm init`](#workflow-when-using-kubeadm-init)
    - [Workflow when using `kubeadm join`](#workflow-when-using-kubeadm-join)
  - [The kubelet drop-in file for systemd](#the-kubelet-drop-in-file-for-systemd)
      - [Note:](#note)
- [This is a file that "kubeadm init" and "kubeadm join" generate at runtime, populating](#this-is-a-file-that-kubeadm-init-and-kubeadm-join-generate-at-runtime-populating)
- [This is a file that the user can use for overrides of the kubelet args as a last resort. Preferably,](#this-is-a-file-that-the-user-can-use-for-overrides-of-the-kubelet-args-as-a-last-resort-preferably)
- [the user should use the .NodeRegistration.KubeletExtraArgs object in the configuration files instead.](#the-user-should-use-the-noderegistrationkubeletextraargs-object-in-the-configuration-files-instead)
- [KUBELET\_EXTRA\_ARGS should be sourced from this file.](#kubeletextraargs-should-be-sourced-from-this-file)
  - [Feedback](#feedback)

---

Configuring each kubelet in your cluster using kubeadm | Kubernetes