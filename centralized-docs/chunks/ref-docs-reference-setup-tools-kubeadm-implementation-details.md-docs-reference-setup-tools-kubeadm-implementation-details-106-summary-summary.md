---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#106-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 113
summary: * Runs preflight checks similarly to `kubeadm upgrade apply`. * For control plane nodes, upgrades the control plane manifest files on disk in `/etc/kubernetes/manifests` and waits for the kubelet to...
---

* Runs preflight checks similarly to `kubeadm upgrade apply`.
* For control plane nodes, upgrades the control plane manifest files on disk in `/etc/kubernetes/manifests`
and waits for the kubelet to restart the components if the files have changed.
* Writes updated kubelet configuration for this node in `/var/lib/kubelet/config.yaml`,
and read the node's `/var/lib/kubelet/instance-config.yaml` file and
patch fields like `containerRuntimeEndpoint`
from this instance configuration into `/var/lib/kubelet/config.yaml`.