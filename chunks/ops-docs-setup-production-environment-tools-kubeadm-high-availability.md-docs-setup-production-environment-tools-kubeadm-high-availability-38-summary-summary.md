---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#38-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 54
summary: ### Steps for the rest of the control plane nodes For each additional control plane node you should: 1. Execute the join command that was previously given to you by the `kubeadm init` output on the...
---

### Steps for the rest of the control plane nodes
For each additional control plane node you should:
1. Execute the join command that was previously given to you by the `kubeadm init` output on the first node.
It should look something like this: