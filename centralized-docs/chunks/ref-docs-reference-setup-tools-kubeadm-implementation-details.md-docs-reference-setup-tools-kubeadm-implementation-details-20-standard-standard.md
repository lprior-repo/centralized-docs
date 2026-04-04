---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#20-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 465
summary: ### Configure TLS-Bootstrapping for node joining Kubeadm uses [Authenticating with Bootstrap Tokens](/docs/reference/access-authn-authz/bootstrap-tokens/) for joining new nodes to an existing...
---

### Configure TLS-Bootstrapping for node joining
Kubeadm uses [Authenticating with Bootstrap Tokens](/docs/reference/access-authn-authz/bootstrap-tokens/)
for joining new nodes to an existing cluster; for more details see also
[design proposal](https://git.k8s.io/design-proposals-archive/cluster-lifecycle/bootstrap-discovery.md).
`kubeadm init` ensures that everything is properly configured for this process, and this includes
following steps as well as setting API server and controller flags as already described in
previous paragraphs.
#### Note:
TLS bootstrapping for nodes can be configured with the command
[`kubeadm init phase bootstrap-token`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-bootstrap-token),
executing all the configuration steps described in following paragraphs;
alternatively, each step can be invoked individually.
#### Create a bootstrap token
`kubeadm init` creates a first bootstrap token, either generated automatically or provided by the
user with the `--token` flag; as documented in bootstrap token specification, token should be
saved as a secret with name `bootstrap-token-&lt;token-id&gt;` under `kube-system` namespace.
Please note that:
1. The default token created by `kubeadm init` will be used to validate temporary user during TLS
bootstrap process; those users will be member of
`system:bootstrappers:kubeadm:default-node-token` group
2. The token has a limited validity, default 24 hours (the interval may be changed with the `—token-ttl` flag)
3. Additional tokens can be created with the [`kubeadm token`](/docs/reference/setup-tools/kubeadm/kubeadm-token/)
command, that provide other useful functions for token management as well.#### Allow joining nodes to call CSR API
Kubeadm ensures that users in `system:bootstrappers:kubeadm:default-node-token` group are able to
access the certificate signing API.
This is implemented by creating a ClusterRoleBinding named `kubeadm:kubelet-bootstrap` between the
group above and the default RBAC role `system:node-bootstrapper`.