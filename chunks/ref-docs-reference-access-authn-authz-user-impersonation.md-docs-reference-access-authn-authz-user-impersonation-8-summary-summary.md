---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#8-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 114
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