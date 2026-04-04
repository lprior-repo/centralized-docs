---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#45-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 105
summary: 5. Generation of kubeconfig files can be invoked individually with the [`kubeadm init phase kubeconfig all`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-kubeconfig) command###...
---

5. Generation of kubeconfig files can be invoked individually with the
[`kubeadm init phase kubeconfig all`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-kubeconfig) command### Generate static Pod manifests for control plane components
Kubeadm writes static Pod manifest files for control plane components to
`/etc/kubernetes/manifests`. The kubelet watches this directory for Pods to be created on startup.
Static Pod manifests share a set of common properties: