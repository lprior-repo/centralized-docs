---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#41-summary
chunk_level: summary
chunk_type: table
heading: The kubelet drop-in file for systemd
token_count: 128
summary: |Package name|Description| |`kubeadm`|Installs the `/usr/bin/kubeadm` CLI tool and the [kubelet drop-in file](#the-kubelet-drop-in-file-for-systemd) for the kubelet.| |`kubelet`|Installs the...
---

|Package name|Description|
|`kubeadm`|Installs the `/usr/bin/kubeadm` CLI tool and the [kubelet drop-in file](#the-kubelet-drop-in-file-for-systemd) for the kubelet.|
|`kubelet`|Installs the `/usr/bin/kubelet` binary.|
|`kubectl`|Installs the `/usr/bin/kubectl` binary.|
|`cri-tools`|Installs the `/usr/bin/crictl` binary from the [cri-tools git repository](https://github.com/kubernetes-sigs/cri-tools).|