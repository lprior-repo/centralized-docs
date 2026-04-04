---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#23-summary
chunk_level: summary
chunk_type: prose
heading: ConfigMap Signing
token_count: 89
summary: #### Warning: Any party with a bootstrapping token can create a valid signature for that token. When using ConfigMap signing it's discouraged to share the same token with many clients, since a...
---

#### Warning:
Any party with a bootstrapping token can create a valid signature for that
token. When using ConfigMap signing it's discouraged to share the same token with
many clients, since a compromised client can potentially man-in-the middle another
client relying on the signature to bootstrap TLS trust.
Consult the [kubeadm implementation details](/docs/reference/setup-tools/kubeadm/implementation-details/)
section for more information.