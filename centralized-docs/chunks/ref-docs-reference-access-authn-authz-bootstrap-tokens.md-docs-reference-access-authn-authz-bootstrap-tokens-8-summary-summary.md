---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#8-summary
chunk_level: summary
chunk_type: prose
heading: Enabling Bootstrap Token Authentication
token_count: 100
summary: ``` `Authorization: Bearer 07401b.f395accd246ae52d ` ``` Tokens authenticate as the username `system:bootstrap:&lt;token id&gt;` and are members of the group `system:bootstrappers`. Additional groups...
---

```
`Authorization: Bearer 07401b.f395accd246ae52d
`
```
Tokens authenticate as the username `system:bootstrap:&lt;token id&gt;` and are members
of the group `system:bootstrappers`.
Additional groups may be specified in the token's Secret.
Expired tokens can be deleted automatically by enabling the `tokencleaner`
controller on the controller manager.
```
`--controllers=\*,tokencleaner
`
```