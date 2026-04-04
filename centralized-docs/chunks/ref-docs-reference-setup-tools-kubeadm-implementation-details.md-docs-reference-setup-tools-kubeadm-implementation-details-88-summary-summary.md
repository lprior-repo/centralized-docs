---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#88-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm join phases internal design
token_count: 73
summary: #### Shared token discovery If `kubeadm join` is invoked with `--discovery-token`, token discovery is used; in this case the node basically retrieves the cluster CA certificates from the...
---

#### Shared token discovery
If `kubeadm join` is invoked with `--discovery-token`, token discovery is used; in this case the
node basically retrieves the cluster CA certificates from the `cluster-info` ConfigMap in the
`kube-public` namespace.
In order to prevent "man in the middle" attacks, several steps are taken: