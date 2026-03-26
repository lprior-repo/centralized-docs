---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#34-summary
chunk_level: summary
chunk_type: prose
heading: The kubelet drop-in file for systemd
token_count: 57
summary: `/etc/systemd/system/kubelet.service.d/local-overrides.conf` to override the unit settings configured by `kubeadm`. Here is what you are likely to find in...
---

`/etc/systemd/system/kubelet.service.d/local-overrides.conf`
to override the unit settings configured by `kubeadm`.
Here is what you are likely to find in `/usr/lib/systemd/system/kubelet.service.d/10-kubeadm.conf`: