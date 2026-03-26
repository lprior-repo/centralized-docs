---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#14-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 92
summary: * Three or more machines that meet [kubeadm's minimum requirements](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#before-you-begin) for the control-plane nodes. Having an odd...
---

* Three or more machines that meet [kubeadm's minimum requirements](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#before-you-begin) for
the control-plane nodes. Having an odd number of control plane nodes can help
with leader selection in the case of machine or zone failure.
* including a [container runtime](/docs/setup/production-environment/container-runtimes), already set up and working