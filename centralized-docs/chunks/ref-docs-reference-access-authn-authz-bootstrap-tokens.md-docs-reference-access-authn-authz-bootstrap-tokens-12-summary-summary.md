---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#12-summary
chunk_level: summary
chunk_type: prose
heading: Bootstrap Token Secret Format
token_count: 118
summary: # Extra groups to authenticate the token as. Must start with \"system:bootstrappers:\" auth-extra-groups: system:bootstrappers:worker,system:bootstrappers:ingress ` ``` The type of the secret must be...
---

# Extra groups to authenticate the token as. Must start with "system:bootstrappers:"
auth-extra-groups: system:bootstrappers:worker,system:bootstrappers:ingress
`
```
The type of the secret must be `bootstrap.kubernetes.io/token` and the name must
be `bootstrap-token-&lt;token id&gt;`. It must also exist in the `kube-system` namespace.
The `usage-bootstrap-\*` members indicate what this secret is intended to be used for.
A value must be set to `true` to be enabled.