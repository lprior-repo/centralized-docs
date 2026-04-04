---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#22-summary
chunk_level: summary
chunk_type: prose
heading: ConfigMap Signing
token_count: 77
summary: is then used to form a whole JWS by inserting it between the 2 dots. You can verify the JWS using the `HS256` scheme (HMAC-SHA256) with the full token (e.g. `07401b.f395accd246ae52d`) as the shared...
---

is then used to form a whole JWS by inserting it between the 2 dots. You can
verify the JWS using the `HS256` scheme (HMAC-SHA256) with the full token (e.g.
`07401b.f395accd246ae52d`) as the shared secret. Users *must* verify that HS256
is used.