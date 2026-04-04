---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#25-standard
chunk_level: standard
chunk_type: prose
heading: TLS Bootstrap
token_count: 457
summary: #### Note: You can skip CA validation by passing the `--discovery-token-unsafe-skip-ca-verification` flag on the command line. This weakens the kubeadm security model since others can potentially...
---

#### Note:
You can skip CA validation by passing the `--discovery-token-unsafe-skip-ca-verification` flag on the command line.
This weakens the kubeadm security model since others can potentially impersonate the Kubernetes API server.
#### File/https discovery
If `kubeadm join` is invoked with `--discovery-file`, file discovery is used; this file can be a
local file or downloaded via an HTTPS URL; in case of HTTPS, the host installed CA bundle is used
to verify the connection.
With file discovery, the cluster CA certificate is provided into the file itself; in fact, the
discovery file is a kubeconfig file with only `server` and `certificate-authority-data` attributes
set, as described in the [`kubeadm join`](/docs/reference/setup-tools/kubeadm/kubeadm-join/#file-or-https-based-discovery)
reference doc; when the connection with the cluster is established, kubeadm tries to access the
`cluster-info` ConfigMap, and if available, uses it.
## TLS Bootstrap
Once the cluster info is known, the file `bootstrap-kubelet.conf` is written, thus allowing
kubelet to do TLS Bootstrapping.
The TLS bootstrap mechanism uses the shared token to temporarily authenticate with the Kubernetes
API server to submit a certificate signing request (CSR) for a locally created key pair.
The request is then automatically approved and the operation completes saving `ca.crt` file and
`kubelet.conf` file to be used by the kubelet for joining the cluster, while `bootstrap-kubelet.conf`
is deleted.
#### Note:
* The temporary authentication is validated against the token saved during the `kubeadm init`
process (or with additional tokens created with `kubeadm token` command)
* The temporary authentication resolves to a user member of
`system:bootstrappers:kubeadm:default-node-token` group which was granted access to the CSR api
during the `kubeadm init` process
* The automatic CSR approval is managed by the csrapprover controller, according to
the configuration present in the `kubeadm init` process