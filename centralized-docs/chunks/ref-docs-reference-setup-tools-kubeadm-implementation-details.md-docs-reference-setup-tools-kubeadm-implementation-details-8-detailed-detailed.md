---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#8-detailed
chunk_level: detailed
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 858
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
#### Set up auto approval for new bootstrap tokens
Kubeadm ensures that the Bootstrap Token will get its CSR request automatically approved by the
csrapprover controller.
This is implemented by creating ClusterRoleBinding named `kubeadm:node-autoapprove-bootstrap`
between the `system:bootstrappers:kubeadm:default-node-token` group and the default role
`system:certificates.k8s.io:certificatesigningrequests:nodeclient`.
The role `system:certificates.k8s.io:certificatesigningrequests:nodeclient` should be created as
well, granting POST permission to
`/apis/certificates.k8s.io/certificatesigningrequests/nodeclient`.
#### Set up nodes certificate rotation with auto approval
Kubeadm ensures that certificate rotation is enabled for nodes, and that a new certificate request
for nodes will get its CSR request automatically approved by the csrapprover controller.
This is implemented by creating ClusterRoleBinding named
`kubeadm:node-autoapprove-certificate-rotation` between the `system:nodes` group and the default
role `system:certificates.k8s.io:certificatesigningrequests:selfnodeclient`.
#### Create the public cluster-info ConfigMap
This phase creates the `cluster-info` ConfigMap in the `kube-public` namespace.
Additionally, it creates a Role and a RoleBinding granting access to the ConfigMap for
unauthenticated users (i.e. users in RBAC group `system:unauthenticated`).
#### Note:
The access to the `cluster-info` ConfigMap *is not* rate-limited. This may or may not be a
problem if you expose your cluster's API server to the internet; worst-case scenario here is a
DoS attack where an attacker uses all the in-flight requests the kube-apiserver can handle to
serve the `cluster-info` ConfigMap.