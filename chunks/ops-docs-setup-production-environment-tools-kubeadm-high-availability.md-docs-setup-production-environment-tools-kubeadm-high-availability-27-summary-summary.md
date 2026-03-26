---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#27-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 85
summary: #### Note: The `kubeadm init` flags `--config` and `--certificate-key` cannot be mixed, therefore if you want to use the [kubeadm configuration](/docs/reference/config-api/kubeadm-config.v1beta4/)...
---

#### Note:
The `kubeadm init` flags `--config` and `--certificate-key` cannot be mixed, therefore if you want
to use the [kubeadm configuration](/docs/reference/config-api/kubeadm-config.v1beta4/)
you must add the `certificateKey` field in the appropriate config locations
(under `InitConfiguration` and `JoinConfiguration: controlPlane`).