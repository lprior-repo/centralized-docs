---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#10-detailed
chunk_level: detailed
chunk_type: prose
heading: TLS Bootstrap
token_count: 1000
summary: ## kubeadm join phases internal design Similarly to `kubeadm init`, also `kubeadm join` internal workflow consists of a sequence of atomic work tasks to perform. This is split into discovery (having...
---

## kubeadm join phases internal design
Similarly to `kubeadm init`, also `kubeadm join` internal workflow consists of a sequence of
atomic work tasks to perform.
This is split into discovery (having the Node trust the Kubernetes API Server) and TLS bootstrap
(having the Kubernetes API Server trust the Node).
see [Authenticating with Bootstrap Tokens](/docs/reference/access-authn-authz/bootstrap-tokens/)
or the corresponding [design proposal](https://git.k8s.io/design-proposals-archive/cluster-lifecycle/bootstrap-discovery.md).
### Preflight checks
`kubeadm` executes a set of preflight checks before starting the join, with the aim to verify
preconditions and avoid common cluster startup problems.
Also note that:
1. `kubeadm join` preflight checks are basically a subset of `kubeadm init` preflight checks
2. If you are joining a Windows node, Linux specific controls are skipped.
3. In any case the user can skip specific preflight checks (or eventually all preflight checks)
with the `--ignore-preflight-errors` option.### Discovery cluster-info
There are 2 main schemes for discovery. The first is to use a shared token along with the IP
address of the API server.
The second is to provide a file (that is a subset of the standard kubeconfig file).
#### Shared token discovery
If `kubeadm join` is invoked with `--discovery-token`, token discovery is used; in this case the
node basically retrieves the cluster CA certificates from the `cluster-info` ConfigMap in the
`kube-public` namespace.
In order to prevent "man in the middle" attacks, several steps are taken:
* First, the CA certificate is retrieved via insecure connection (this is possible because
`kubeadm init` is granted access to `cluster-info` users for `system:unauthenticated`)
* Then the CA certificate goes through following validation steps:
* Basic validation: using the token ID against a JWT signature
* Pub key validation: using provided `--discovery-token-ca-cert-hash`. This value is available
in the output of `kubeadm init` or can be calculated using standard tools (the hash is
calculated over the bytes of the Subject Public Key Info (SPKI) object as in RFC7469). The
`--discovery-token-ca-cert-hash flag` may be repeated multiple times to allow more than one public key.
* As an additional validation, the CA certificate is retrieved via secure connection and then
compared with the CA retrieved initially
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