---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#31-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 115
summary: * The node-name * The `--apiserver-advertise-address` * Additional alternative names specified by the user * A client certificate for the API server to connect to the kubelets securely, generated...
---

* The node-name
* The `--apiserver-advertise-address`
* Additional alternative names specified by the user
* A client certificate for the API server to connect to the kubelets securely, generated using
`ca.crt` as the CA and saved into `apiserver-kubelet-client.crt` file with its private key
`apiserver-kubelet-client.key`.
This certificate should be in the `system:masters` organization
* A private key for signing ServiceAccount Tokens saved into `sa.key` file along with its public key `sa.pub`