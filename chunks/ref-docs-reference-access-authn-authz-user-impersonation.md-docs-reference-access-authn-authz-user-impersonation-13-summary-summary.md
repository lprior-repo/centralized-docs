---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#13-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 95
summary: #### Note: `Impersonate-Uid` is only available in versions 1.22.0 and higher. An example of the impersonation headers used when impersonating a user with groups: ``` `Impersonate-User:...
---

#### Note:
`Impersonate-Uid` is only available in versions 1.22.0 and higher.
An example of the impersonation headers used when impersonating a user with groups:
```
`Impersonate-User: jane.doe@example.com
Impersonate-Group: developers
Impersonate-Group: admins
`
```
An example of the impersonation headers used when impersonating a user with a UID and
extra fields: