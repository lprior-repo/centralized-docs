---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#34-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 126
summary: from the API server, to verify presented JWTs without requiring use of the TokenReview API. Services that verify JWTs in this way **do not verify** the claims embedded in the JWT token to be current...
---

from the API server, to verify presented JWTs without requiring use of the TokenReview API.
Services that verify JWTs in this way **do not verify** the claims embedded in the JWT token to be
current and still valid.
This means if the token is bound to an object, and that object no longer exists, the token will still
be considered valid (until the configured token expires).
Clients that require assurance that a token's bound claims are still valid **MUST** use the TokenReview
API to present the token to the `kube-apiserver` for it to verify and expand the embedded claims, using