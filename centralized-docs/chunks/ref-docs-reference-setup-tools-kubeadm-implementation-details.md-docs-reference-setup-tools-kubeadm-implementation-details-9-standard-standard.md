---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#9-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 407
summary: ### Generate the necessary certificates Kubeadm generates certificate and private key pairs for different purposes: * A self signed certificate authority for the Kubernetes cluster saved into...
---

### Generate the necessary certificates
Kubeadm generates certificate and private key pairs for different purposes:
* A self signed certificate authority for the Kubernetes cluster saved into `ca.crt` file and
`ca.key` private key file
* A serving certificate for the API server, generated using `ca.crt` as the CA, and saved into
`apiserver.crt` file with its private key `apiserver.key`. This certificate should contain
the following alternative names:
* The Kubernetes service's internal clusterIP (the first address in the services CIDR, e.g.
`10.96.0.1` if service subnet is `10.96.0.0/12`)
* Kubernetes DNS names, e.g. `kubernetes.default.svc.cluster.local` if `--service-dns-domain`
flag value is `cluster.local`, plus default DNS names `kubernetes.default.svc`,
`kubernetes.default`, `kubernetes`
* The node-name
* The `--apiserver-advertise-address`
* Additional alternative names specified by the user
* A client certificate for the API server to connect to the kubelets securely, generated using
`ca.crt` as the CA and saved into `apiserver-kubelet-client.crt` file with its private key
`apiserver-kubelet-client.key`.
This certificate should be in the `system:masters` organization
* A private key for signing ServiceAccount Tokens saved into `sa.key` file along with its public key `sa.pub`
* A certificate authority for the front proxy saved into `front-proxy-ca.crt` file with its key
`front-proxy-ca.key`
* A client certificate for the front proxy client, generated using `front-proxy-ca.crt` as the CA and
saved into `front-proxy-client.crt` file with its private key`front-proxy-client.key`
Certificates are stored by default in `/etc/kubernetes/pki`, but this directory is configurable
using the `--cert-dir` flag.
Please note that: