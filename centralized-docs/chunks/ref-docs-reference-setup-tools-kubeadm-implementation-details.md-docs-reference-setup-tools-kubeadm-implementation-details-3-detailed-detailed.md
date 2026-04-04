---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#3-detailed
chunk_level: detailed
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 750
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
1. If a given certificate and private key pair both exist, and their content is evaluated to be compliant with the above specs, the existing files will
be used and the generation phase for the given certificate will be skipped. This means the user can, for example, copy an existing CA to
`/etc/kubernetes/pki/ca.{crt,key}`, and then kubeadm will use those files for signing the rest of the certs.
See also [using custom certificates](/docs/tasks/administer-cluster/kubeadm/kubeadm-certs/#custom-certificates)
2. For the CA, it is possible to provide the `ca.crt` file but not the `ca.key` file. If all other certificates and kubeconfig files
are already in place, kubeadm recognizes this condition and activates the ExternalCA, which also implies the `csrsigner` controller in
controller-manager won't be started
3. If kubeadm is running in [external CA mode](/docs/tasks/administer-cluster/kubeadm/kubeadm-certs/#external-ca-mode);
all the certificates must be provided by the user, because kubeadm cannot generate them by itself
4. In case kubeadm is executed in the `--dry-run` mode, certificate files are written in a temporary folder
5. Certificate generation can be invoked individually with the
[`kubeadm init phase certs all`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-certs) command### Generate kubeconfig files for control plane components
Kubeadm generates kubeconfig files with identities for control plane components: