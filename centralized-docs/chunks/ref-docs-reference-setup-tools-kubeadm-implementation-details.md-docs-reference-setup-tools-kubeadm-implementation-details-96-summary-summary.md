---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#96-summary
chunk_level: summary
chunk_type: prose
heading: TLS Bootstrap
token_count: 116
summary: #### Note: * The temporary authentication is validated against the token saved during the `kubeadm init` process (or with additional tokens created with `kubeadm token` command) * The temporary...
---

#### Note:
* The temporary authentication is validated against the token saved during the `kubeadm init`
process (or with additional tokens created with `kubeadm token` command)
* The temporary authentication resolves to a user member of
`system:bootstrappers:kubeadm:default-node-token` group which was granted access to the CSR api
during the `kubeadm init` process
* The automatic CSR approval is managed by the csrapprover controller, according to
the configuration present in the `kubeadm init` process