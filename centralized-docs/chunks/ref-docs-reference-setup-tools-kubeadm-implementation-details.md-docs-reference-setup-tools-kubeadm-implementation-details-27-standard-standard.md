---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#27-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 46
summary: ### kubeadm upgrade apply `kubeadm upgrade apply` prepares the cluster for the upgrade of all nodes, and also upgrades the control plane node where it's run. The steps it performs are:
---

### kubeadm upgrade apply
`kubeadm upgrade apply` prepares the cluster for the upgrade of all nodes, and also
upgrades the control plane node where it's run. The steps it performs are: