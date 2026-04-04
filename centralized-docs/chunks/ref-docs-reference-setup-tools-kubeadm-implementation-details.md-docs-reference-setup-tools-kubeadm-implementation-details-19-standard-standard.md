---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#19-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 283
summary: ### Save the kubeadm ClusterConfiguration in a ConfigMap for later reference kubeadm saves the configuration passed to `kubeadm init` in a ConfigMap named `kubeadm-config` under `kube-system`...
---

### Save the kubeadm ClusterConfiguration in a ConfigMap for later reference
kubeadm saves the configuration passed to `kubeadm init` in a ConfigMap named `kubeadm-config`
under `kube-system` namespace.
This will ensure that kubeadm actions executed in future (e.g `kubeadm upgrade`) will be able to
determine the actual/current cluster state and make new decisions based on that data.
Please note that:
1. Before saving the ClusterConfiguration, sensitive information like the token is stripped from the configuration
2. Upload of control plane node configuration can be invoked individually with the command
[`kubeadm init phase upload-config`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-upload-config).### Mark the node as control-plane
As soon as the control plane is available, kubeadm executes the following actions:
* Labels the node as control-plane with `node-role.kubernetes.io/control-plane=""`
* Taints the node with `node-role.kubernetes.io/control-plane:NoSchedule`
Please note that the phase to mark the control-plane phase can be invoked
individually with the [`kubeadm init phase mark-control-plane`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-mark-control-plane) command.