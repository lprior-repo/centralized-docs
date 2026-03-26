---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#28-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 112
summary: #### Note: Some CNI network plugins require additional configuration, for example specifying the pod IP CIDR, while others do not. See the [CNI network...
---

#### Note:
Some CNI network plugins require additional configuration, for example specifying the pod IP CIDR, while others do not.
See the [CNI network documentation](/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/#pod-network).
To add a pod CIDR pass the flag `--pod-network-cidr`, or if you are using a kubeadm configuration file
set the `podSubnet` field under the `networking` object of `ClusterConfiguration`.
The output looks similar to: