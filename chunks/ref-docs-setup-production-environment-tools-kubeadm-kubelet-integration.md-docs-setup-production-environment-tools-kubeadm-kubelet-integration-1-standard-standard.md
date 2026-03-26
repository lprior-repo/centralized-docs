---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 319
summary: # Configuring each kubelet in your cluster using kubeadm **Note:** Dockershim has been removed from the Kubernetes project as of release 1.24. Read the [Dockershim Removal FAQ](/dockershim) for...
---

# Configuring each kubelet in your cluster using kubeadm
**Note:** Dockershim has been removed from the Kubernetes project as of release 1.24. Read the [Dockershim Removal FAQ](/dockershim) for further details.
FEATURE STATE:
`Kubernetes v1.11 [stable]`
The lifecycle of the kubeadm CLI tool is decoupled from the
[kubelet](/docs/reference/command-line-tools-reference/kubelet/), which is a daemon that runs
on each node within the Kubernetes cluster. The kubeadm CLI tool is executed by the user when Kubernetes is
initialized or upgraded, whereas the kubelet is always running in the background.
Since the kubelet is a daemon, it needs to be maintained by some kind of an init
system or service manager. When the kubelet is installed using DEBs or RPMs,
systemd is configured to manage the kubelet. You can use a different service
manager instead, but you need to configure it manually.
Some kubelet configuration details need to be the same across all kubelets involved in the cluster, while
other configuration aspects need to be set on a per-kubelet basis to accommodate the different
characteristics of a given machine (such as OS, storage, and networking). You can manage the configuration
of your kubelets manually, but kubeadm now provides a `KubeletConfiguration` API type for
[managing your kubelet configurations centrally](#configure-kubelets-using-kubeadm).