---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#29-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 120
summary: * A self signed certificate authority for the Kubernetes cluster saved into `ca.crt` file and `ca.key` private key file * A serving certificate for the API server, generated using `ca.crt` as the CA,...
---

* A self signed certificate authority for the Kubernetes cluster saved into `ca.crt` file and
`ca.key` private key file
* A serving certificate for the API server, generated using `ca.crt` as the CA, and saved into
`apiserver.crt` file with its private key `apiserver.key`. This certificate should contain
the following alternative names:
* The Kubernetes service's internal clusterIP (the first address in the services CIDR, e.g.
`10.96.0.1` if service subnet is `10.96.0.0/12`)