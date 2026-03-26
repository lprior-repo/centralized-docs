---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#32-summary
chunk_level: summary
chunk_type: prose
heading: The kubelet drop-in file for systemd
token_count: 112
summary: `kubeadm` ships with configuration for how systemd should run the kubelet. Note that the kubeadm CLI command never touches this drop-in file. This configuration file installed by the `kubeadm`...
---

`kubeadm` ships with configuration for how systemd should run the kubelet.
Note that the kubeadm CLI command never touches this drop-in file.
This configuration file installed by the `kubeadm`
[package](https://github.com/kubernetes/release/blob/cd53840/cmd/krel/templates/latest/kubeadm/10-kubeadm.conf) is written to
`/usr/lib/systemd/system/kubelet.service.d/10-kubeadm.conf` and is used by systemd.
It augments the basic