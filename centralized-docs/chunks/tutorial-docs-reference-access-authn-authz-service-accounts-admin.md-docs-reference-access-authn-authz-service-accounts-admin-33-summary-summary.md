---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#33-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 128
summary: The `aud` and `iss` fields in this JWT may differ between different Kubernetes clusters depending on your configuration. The presence of both the `pod` and `node` claim implies that this token is...
---

The `aud` and `iss` fields in this JWT may differ between different Kubernetes clusters depending
on your configuration.
The presence of both the `pod` and `node` claim implies that this token is bound
to a *Pod* object. When verifying Pod bound ServiceAccount tokens, the API server **does not**
verify the existence of the referenced Node object.
Services that run outside of Kubernetes and want to perform offline validation of JWTs may
use this schema, along with a compliant JWT validator configured with OpenID Discovery information
from the API server, to verify presented JWTs without requiring use of the TokenReview API.