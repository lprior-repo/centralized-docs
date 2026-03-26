---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#5-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 103
summary: - [the user should use the .NodeRegistration.KubeletExtraArgs object in the configuration files...
---

- [the user should use the .NodeRegistration.KubeletExtraArgs object in the configuration files instead.](#the-user-should-use-the-noderegistrationkubeletextraargs-object-in-the-configuration-files-instead)
- [KUBELET\_EXTRA\_ARGS should be sourced from this file.](#kubeletextraargs-should-be-sourced-from-this-file)
  - [Feedback](#feedback)

---

Configuring each kubelet in your cluster using kubeadm | Kubernetes