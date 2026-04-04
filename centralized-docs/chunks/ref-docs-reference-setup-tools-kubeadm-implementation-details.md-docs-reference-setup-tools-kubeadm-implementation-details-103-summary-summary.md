---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#103-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 108
summary: * Writes updated kubelet configuration for this node in `/var/lib/kubelet/config.yaml`, and read the node's `/var/lib/kubelet/instance-config.yaml` file and patch fields like...
---

* Writes updated kubelet configuration for this node in `/var/lib/kubelet/config.yaml`,
and read the node's `/var/lib/kubelet/instance-config.yaml` file
and patch fields like `containerRuntimeEndpoint`
from this instance configuration into `/var/lib/kubelet/config.yaml`.
* Configures bootstrap token and the `cluster-info` ConfigMap for RBAC rules. This is the same as
in the `kubeadm init` stage and ensures that the cluster continues to support nodes joining with bootstrap tokens.