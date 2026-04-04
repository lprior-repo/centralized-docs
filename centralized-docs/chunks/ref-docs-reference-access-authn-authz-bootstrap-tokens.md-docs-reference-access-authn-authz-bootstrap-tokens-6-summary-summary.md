---
doc_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens
chunk_id: ref/docs-reference-access-authn-authz-bootstrap-tokens.md/docs-reference-access-authn-authz-bootstrap-tokens#6-summary
chunk_level: summary
chunk_type: prose
heading: Token Format
token_count: 103
summary: ## Token Format Bootstrap Tokens take the form of `abcdef.0123456789abcdef`. More formally, they must match the regular expression `[a-z0-9]{6}\\.[a-z0-9]{16}`. The first part of the token is the...
---

## Token Format
Bootstrap Tokens take the form of `abcdef.0123456789abcdef`.
More formally, they must match the regular expression `[a-z0-9]{6}\\.[a-z0-9]{16}`.
The first part of the token is the "Token ID" and is considered public
information. It is used when referring to a token without leaking the secret
part used for authentication. The second part is the "Token Secret" and should
only be shared with trusted parties.