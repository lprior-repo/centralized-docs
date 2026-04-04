---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#7-summary
chunk_level: summary
chunk_type: prose
heading: Enabling Bootstrap Token Authentication
token_count: 82
summary: ## Enabling Bootstrap Token Authentication The Bootstrap Token authenticator can be enabled using the following flag on the API server: ``` `--enable-bootstrap-token-auth ` ``` When enabled,...
---

## Enabling Bootstrap Token Authentication
The Bootstrap Token authenticator can be enabled using the following flag on the
API server:
```
`--enable-bootstrap-token-auth
`
```
When enabled, bootstrapping tokens can be used as bearer token credentials to
authenticate requests against the API server.
```
`Authorization: Bearer 07401b.f395accd246ae52d
`
```