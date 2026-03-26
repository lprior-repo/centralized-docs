---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#52-summary
chunk_level: summary
chunk_type: prose
heading: External etcd nodes
token_count: 99
summary: ### Steps for the rest of the control plane nodes The steps are the same as for the stacked etcd setup: * Make sure the first control plane node is fully initialized. * Join each control plane node...
---

### Steps for the rest of the control plane nodes
The steps are the same as for the stacked etcd setup:
* Make sure the first control plane node is fully initialized.
* Join each control plane node with the join command you saved to a text file. It's recommended
to join the control plane nodes one at a time.
* Don't forget that the decryption key from `--certificate-key` expires after two hours, by default.## Common tasks after bootstrapping control plane