---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#16-summary
chunk_level: summary
chunk_type: prose
heading: ConfigMap Signing
token_count: 85
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