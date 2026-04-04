---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#98-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 119
summary: `kubeadm upgrade` has sub-commands for handling the upgrade of the Kubernetes cluster created by kubeadm. You must run `kubeadm upgrade apply` on a control plane node (you can choose which one); this...
---

`kubeadm upgrade` has sub-commands for handling the upgrade of the Kubernetes cluster created by kubeadm.
You must run `kubeadm upgrade apply` on a control plane node (you can choose which one);
this starts the upgrade process. You then run `kubeadm upgrade node` on all remaining
nodes (both worker nodes and control plane nodes).
Both `kubeadm upgrade apply` and `kubeadm upgrade node` have a `phase` subcommand which provides access
to the internal phases of the upgrade process.
See