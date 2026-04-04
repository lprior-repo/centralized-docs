---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#36-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 107
summary: * A kubeconfig file for the kubelet to use during TLS bootstrap - `/etc/kubernetes/bootstrap-kubelet.conf`. Inside this file, there is a bootstrap-token or embedded client certificates for...
---

* A kubeconfig file for the kubelet to use during TLS bootstrap -
`/etc/kubernetes/bootstrap-kubelet.conf`. Inside this file, there is a bootstrap-token or embedded
client certificates for authenticating this node with the cluster.
This client certificate should:
* Be in the `system:nodes` organization, as required by the
[Node Authorization](/docs/reference/access-authn-authz/node/) module
* Have the Common Name (CN) `system:node:&lt;hostname-lowercased&gt;`