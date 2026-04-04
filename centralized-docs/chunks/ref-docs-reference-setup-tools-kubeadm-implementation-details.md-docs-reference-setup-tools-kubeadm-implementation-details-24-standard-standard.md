---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#24-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm join phases internal design
token_count: 475
summary: ### Preflight checks `kubeadm` executes a set of preflight checks before starting the join, with the aim to verify preconditions and avoid common cluster startup problems. Also note that: 1. `kubeadm...
---

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