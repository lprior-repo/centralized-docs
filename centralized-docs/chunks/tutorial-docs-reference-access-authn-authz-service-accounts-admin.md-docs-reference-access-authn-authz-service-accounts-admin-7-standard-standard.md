---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#7-standard
chunk_level: standard
chunk_type: prose
heading: User accounts versus service accounts
token_count: 316
summary: #### Note: The `aud` and `iss` fields in this JWT may differ between different Kubernetes clusters depending on your configuration. The presence of both the `pod` and `node` claim implies that this...
---

#### Note:
The `aud` and `iss` fields in this JWT may differ between different Kubernetes clusters depending
on your configuration.
The presence of both the `pod` and `node` claim implies that this token is bound
to a *Pod* object. When verifying Pod bound ServiceAccount tokens, the API server **does not**
verify the existence of the referenced Node object.
Services that run outside of Kubernetes and want to perform offline validation of JWTs may
use this schema, along with a compliant JWT validator configured with OpenID Discovery information
from the API server, to verify presented JWTs without requiring use of the TokenReview API.
Services that verify JWTs in this way **do not verify** the claims embedded in the JWT token to be
current and still valid.
This means if the token is bound to an object, and that object no longer exists, the token will still
be considered valid (until the configured token expires).
Clients that require assurance that a token's bound claims are still valid **MUST** use the TokenReview
API to present the token to the `kube-apiserver` for it to verify and expand the embedded claims, using
similar steps to the [Verifying and inspecting private claims](#verifying-and-inspecting-private-claims)
section above, but with a [supported client library](/docs/reference/using-api/client-libraries/).
For more information on JWTs and their structure, see the [JSON Web Token RFC](https://datatracker.ietf.org/doc/html/rfc7519).