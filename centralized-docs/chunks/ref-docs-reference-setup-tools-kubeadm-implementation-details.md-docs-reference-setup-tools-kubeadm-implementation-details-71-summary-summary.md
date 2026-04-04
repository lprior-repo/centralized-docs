---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#71-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 75
summary: #### Create a bootstrap token `kubeadm init` creates a first bootstrap token, either generated automatically or provided by the user with the `--token` flag; as documented in bootstrap token...
---

#### Create a bootstrap token
`kubeadm init` creates a first bootstrap token, either generated automatically or provided by the
user with the `--token` flag; as documented in bootstrap token specification, token should be
saved as a secret with name `bootstrap-token-&lt;token-id&gt;` under `kube-system` namespace.
Please note that: