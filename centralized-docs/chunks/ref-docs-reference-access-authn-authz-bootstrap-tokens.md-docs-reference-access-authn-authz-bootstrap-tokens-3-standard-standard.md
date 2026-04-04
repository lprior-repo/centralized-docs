---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#3-standard
chunk_level: standard
chunk_type: prose
heading: ConfigMap Signing
token_count: 500
summary: ## ConfigMap Signing In addition to authentication, the tokens can be used to sign a ConfigMap. This is used early in a cluster bootstrap process before the client trusts the API server. The signed...
---

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