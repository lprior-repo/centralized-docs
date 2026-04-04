---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#94-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm join phases internal design
token_count: 94
summary: discovery file is a kubeconfig file with only `server` and `certificate-authority-data` attributes set, as described in the [`kubeadm...
---

discovery file is a kubeconfig file with only `server` and `certificate-authority-data` attributes
set, as described in the [`kubeadm join`](/docs/reference/setup-tools/kubeadm/kubeadm-join/#file-or-https-based-discovery)
reference doc; when the connection with the cluster is established, kubeadm tries to access the
`cluster-info` ConfigMap, and if available, uses it.