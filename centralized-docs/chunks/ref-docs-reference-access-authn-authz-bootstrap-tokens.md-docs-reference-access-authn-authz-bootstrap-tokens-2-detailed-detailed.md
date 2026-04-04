---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 945
summary: ## Token Management with kubeadm You can use the `kubeadm` tool to manage tokens on a running cluster. See the [kubeadm token docs](/docs/reference/setup-tools/kubeadm/kubeadm-token/) for details. ##...
---

## Token Management with kubeadm
You can use the `kubeadm` tool to manage tokens on a running cluster. See the
[kubeadm token docs](/docs/reference/setup-tools/kubeadm/kubeadm-token/) for details.
## ConfigMap Signing
In addition to authentication, the tokens can be used to sign a ConfigMap.
This is used early in a cluster bootstrap process before the client trusts the API
server. The signed ConfigMap can be authenticated by the shared token.
Enable ConfigMap signing by enabling the `bootstrapsigner` controller on the
Controller Manager.
```
`--controllers=\*,bootstrapsigner
`
```
The ConfigMap that is signed is `cluster-info` in the `kube-public` namespace.
The typical flow is that a client reads this ConfigMap while unauthenticated and
ignoring TLS errors. It then validates the payload of the ConfigMap by looking
at a signature embedded in the ConfigMap.
The ConfigMap may look like this:
```
`apiVersion: v1
kind: ConfigMap
metadata:
name: cluster-info
namespace: kube-public
data:
jws-kubeconfig-07401b: eyJhbGciOiJIUzI1NiIsImtpZCI6IjA3NDAxYiJ9..tYEfbo6zDNo40MQE07aZcQX2m3EB2rO3NuXtxVMYm9U
kubeconfig: |
apiVersion: v1
clusters:
- cluster:
certificate-authority-data: &lt;really long certificate data&gt;
server: https://10.138.0.2:6443
name: ""
contexts: []
current-context: ""
kind: Config
preferences: {}
users: []
`
```
The `kubeconfig` member of the ConfigMap is a config file with only the cluster
information filled out. The key thing being communicated here is the
`certificate-authority-data`. This may be expanded in the future.
The signature is a JWS signature using the "detached" mode. To validate the
signature, the user should encode the `kubeconfig` payload according to JWS
rules (base64 encoded while discarding any trailing `=`). That encoded payload
is then used to form a whole JWS by inserting it between the 2 dots. You can
verify the JWS using the `HS256` scheme (HMAC-SHA256) with the full token (e.g.
`07401b.f395accd246ae52d`) as the shared secret. Users *must* verify that HS256
is used.
#### Warning:
Any party with a bootstrapping token can create a valid signature for that
token. When using ConfigMap signing it's discouraged to share the same token with
many clients, since a compromised client can potentially man-in-the middle another
client relying on the signature to bootstrap TLS trust.
Consult the [kubeadm implementation details](/docs/reference/setup-tools/kubeadm/implementation-details/)
section for more information.
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified September 11, 2024 at 11:29 AM PST: [add hyperlink for RFC3339 in bootstrap-tokens.md (2e7c1d4935)](https://github.com/kubernetes/website/commit/2e7c1d4935e26d202a7a137677e28264a08d6c44)
## Related Pages

- [Implementation details](docs-reference-setup-tools-kubeadm-implementation-details.md)
- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Kubeadm](docs-reference-setup-tools-kubeadm.md)
- [Communication between Nodes and the Control Plane](docs-concepts-architecture-control-plane-node-communication.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)