---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#86-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm join phases internal design
token_count: 44
summary: 1. `kubeadm join` preflight checks are basically a subset of `kubeadm init` preflight checks 2. If you are joining a Windows node, Linux specific controls are skipped.
---

1. `kubeadm join` preflight checks are basically a subset of `kubeadm init` preflight checks
2. If you are joining a Windows node, Linux specific controls are skipped.