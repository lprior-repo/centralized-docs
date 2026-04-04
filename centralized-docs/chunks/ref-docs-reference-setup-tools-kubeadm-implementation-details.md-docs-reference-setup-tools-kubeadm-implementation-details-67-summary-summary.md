---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#67-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 92
summary: 1. Before saving the ClusterConfiguration, sensitive information like the token is stripped from the configuration 2. Upload of control plane node configuration can be invoked individually with the...
---

1. Before saving the ClusterConfiguration, sensitive information like the token is stripped from the configuration
2. Upload of control plane node configuration can be invoked individually with the command
[`kubeadm init phase upload-config`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-upload-config).### Mark the node as control-plane
As soon as the control plane is available, kubeadm executes the following actions: