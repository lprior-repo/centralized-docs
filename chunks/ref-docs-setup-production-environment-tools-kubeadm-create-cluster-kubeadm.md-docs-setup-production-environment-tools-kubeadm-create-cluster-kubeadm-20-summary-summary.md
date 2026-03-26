---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#20-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 127
summary: If the host does not have a default gateway and if a custom IP address is not passed to a Kubernetes component, the component may exit with an error. To configure the API server advertise address for...
---

If the host does not have a default gateway and if a custom IP address is not passed
to a Kubernetes component, the component may exit with an error.
To configure the API server advertise address for control plane nodes created with both
`init` and `join`, the flag `--apiserver-advertise-address` can be used.
Preferably, this option can be set in the [kubeadm API](/docs/reference/config-api/kubeadm-config.v1beta4/)
as `InitConfiguration.localAPIEndpoint` and `JoinConfiguration.controlPlane.localAPIEndpoint`.
For kubelets on all nodes, the