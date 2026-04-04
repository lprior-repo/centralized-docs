---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#50-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 103
summary: 2. In case kubeadm is executed in the `--dry-run` mode, static Pod files are written in a temporary folder 3. Static Pod manifest generation for control plane components can be invoked individually...
---

2. In case kubeadm is executed in the `--dry-run` mode, static Pod files are written in a
temporary folder
3. Static Pod manifest generation for control plane components can be invoked individually with
the [`kubeadm init phase control-plane all`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-control-plane) command#### API server
The static Pod manifest for the API server is affected by the following parameters provided by the users: