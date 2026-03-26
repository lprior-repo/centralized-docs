---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#28-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 85
summary: 1. (Recommended) If you have plans to upgrade this single control-plane `kubeadm` cluster to [high availability](/docs/setup/production-environment/tools/kubeadm/high-availability/) you should...
---

1. (Recommended) If you have plans to upgrade this single control-plane `kubeadm` cluster
to [high availability](/docs/setup/production-environment/tools/kubeadm/high-availability/)
you should specify the `--control-plane-endpoint` to set the shared endpoint for all control-plane nodes.
Such an endpoint can be either a DNS name or an IP address of a load-balancer.