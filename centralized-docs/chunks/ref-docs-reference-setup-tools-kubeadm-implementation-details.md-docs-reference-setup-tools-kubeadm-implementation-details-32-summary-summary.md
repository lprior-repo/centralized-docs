---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#32-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 125
summary: * A private key for signing ServiceAccount Tokens saved into `sa.key` file along with its public key `sa.pub` * A certificate authority for the front proxy saved into `front-proxy-ca.crt` file with...
---

* A private key for signing ServiceAccount Tokens saved into `sa.key` file along with its public key `sa.pub`
* A certificate authority for the front proxy saved into `front-proxy-ca.crt` file with its key
`front-proxy-ca.key`
* A client certificate for the front proxy client, generated using `front-proxy-ca.crt` as the CA and
saved into `front-proxy-client.crt` file with its private key`front-proxy-client.key`
Certificates are stored by default in `/etc/kubernetes/pki`, but this directory is configurable
using the `--cert-dir` flag.
Please note that: