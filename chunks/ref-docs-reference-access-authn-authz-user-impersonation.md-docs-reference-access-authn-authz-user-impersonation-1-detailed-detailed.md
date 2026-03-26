---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 453
summary: # User Impersonation User *impersonation* is a method of allowing authenticated users to act as another user, group, or service account through HTTP headers. A user can act as another user through...
---

# User Impersonation
User *impersonation* is a method of allowing authenticated users to act as another user,
group, or service account through HTTP headers.
A user can act as another user through impersonation headers. These let requests
manually override the user info a request authenticates as. For example, an admin
could use this feature to debug an authorization policy by temporarily
impersonating another user and seeing if a request was denied.
Impersonation requests first authenticate as the requesting user, then switch
to the impersonated user info.
* A user makes an API call with their credentials *and* impersonation headers.
* API server authenticates the user.
* API server ensures the authenticated users have impersonation privileges.
* Request user info is replaced with impersonation values.
* Request is evaluated, authorization acts on impersonated user info.
The following HTTP headers can be used to performing an impersonation request:
* `Impersonate-User`: The username to act as.
* `Impersonate-Uid`: A unique identifier that represents the user being impersonated. Optional.
Requires "Impersonate-User". Kubernetes does not impose any format requirements on this string.
* `Impersonate-Group`: A group name to act as. Can be provided multiple times to set multiple groups.
Optional. Requires "Impersonate-User".
* `Impersonate-Extra-( extra name )`: A dynamic header used to associate extra fields with the user.
Optional. Requires "Impersonate-User". In order to be preserved consistently, `( extra name )`
must be lower-case, and any characters which aren't [legal in HTTP header labels](https://tools.ietf.org/html/rfc7230#section-3.2.6)
MUST be utf8 and [percent-encoded](https://tools.ietf.org/html/rfc3986#section-2.1).
#### Note:
Prior to 1.11.3 (and 1.10.7, 1.9.11), `( extra name )` could only contain characters which
were [legal in HTTP header labels](https://tools.ietf.org/html/rfc7230#section-3.2.6).