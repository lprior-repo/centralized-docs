---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#4-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 506
summary: #### Note: If two or more default gateways are present on the host, a Kubernetes component will try to use the first one it encounters that has a suitable global unicast IP address. While making this...
---

#### Note:
If two or more default gateways are present on the host, a Kubernetes component will
try to use the first one it encounters that has a suitable global unicast IP address.
While making this choice, the exact ordering of gateways might vary between different
operating systems and kernel versions.
Kubernetes components do not accept custom network interface as an option,
therefore a custom IP address must be passed as a flag to all components instances
that need such a custom configuration.
#### Note:
If the host does not have a default gateway and if a custom IP address is not passed
to a Kubernetes component, the component may exit with an error.
To configure the API server advertise address for control plane nodes created with both
`init` and `join`, the flag `--apiserver-advertise-address` can be used.
Preferably, this option can be set in the [kubeadm API](/docs/reference/config-api/kubeadm-config.v1beta4/)
as `InitConfiguration.localAPIEndpoint` and `JoinConfiguration.controlPlane.localAPIEndpoint`.
For kubelets on all nodes, the `--node-ip` option can be passed in
`.nodeRegistration.kubeletExtraArgs` inside a kubeadm configuration file
(`InitConfiguration` or `JoinConfiguration`).
For dual-stack see
[Dual-stack support with kubeadm](/docs/setup/production-environment/tools/kubeadm/dual-stack-support/).
The IP addresses that you assign to control plane components become part of their X.509 certificates'
subject alternative name fields. Changing these IP addresses would require
signing new certificates and restarting the affected components, so that the change in
certificate files is reflected. See
[Manual certificate renewal](/docs/tasks/administer-cluster/kubeadm/kubeadm-certs/#manual-certificate-renewal)
for more details on this topic.
#### Warning:
The Kubernetes project recommends against this approach (configuring all component instances
with custom IP addresses). Instead, the Kubernetes maintainers recommend to setup the host network,
so that the default gateway IP is the one that Kubernetes components auto-detect and use.
On Linux nodes, you can use commands such as `ip route` to configure networking; your operating
system might also provide higher level network management tools. If your node's default gateway
is a public IP address, you should configure packet filtering or other security measures that
protect the nodes and your cluster.