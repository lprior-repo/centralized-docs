---
doc_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin
chunk_id: tutorial/docs-reference-access-authn-authz-service-accounts-admin.md/docs-reference-access-authn-authz-service-accounts-admin#35-summary
chunk_level: summary
chunk_type: prose
heading: User accounts versus service accounts
token_count: 107
summary: use the TokenReview API to present the token to the `kube-apiserver` for it to verify and expand the embedded claims, using similar steps to the [Verifying and inspecting private...
---

 use the TokenReview
API to present the token to the `kube-apiserver` for it to verify and expand the embedded claims, using
similar steps to the [Verifying and inspecting private claims](#verifying-and-inspecting-private-claims)
section above, but with a [supported client library](/docs/reference/using-api/client-libraries/).
For more information on JWTs and their structure, see the [JSON Web Token RFC](https://datatracker.ietf.org/doc/html/rfc7519).