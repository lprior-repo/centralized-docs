---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#9-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 89
summary: * A user makes an API call with their credentials *and* impersonation headers. * API server authenticates the user. * API server ensures the authenticated users have impersonation privileges. *...
---

* A user makes an API call with their credentials *and* impersonation headers.
* API server authenticates the user.
* API server ensures the authenticated users have impersonation privileges.
* Request user info is replaced with impersonation values.
* Request is evaluated, authorization acts on impersonated user info.
The following HTTP headers can be used to performing an impersonation request:
* `Impersonate-User`: The username to act as.