---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#11-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 110
summary: * `Impersonate-Extra-( extra name )`: A dynamic header used to associate extra fields with the user. Optional. Requires \"Impersonate-User\". In order to be preserved consistently, `( extra name )`...
---

* `Impersonate-Extra-( extra name )`: A dynamic header used to associate extra fields with the user.
Optional. Requires "Impersonate-User". In order to be preserved consistently, `( extra name )`
must be lower-case, and any characters which aren't [legal in HTTP header labels](https://tools.ietf.org/html/rfc7230#section-3.2.6)
MUST be utf8 and [percent-encoded](https://tools.ietf.org/html/rfc3986#section-2.1).