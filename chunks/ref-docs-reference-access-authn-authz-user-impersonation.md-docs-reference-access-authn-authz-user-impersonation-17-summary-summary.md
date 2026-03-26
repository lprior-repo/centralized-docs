---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#17-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 102
summary: ``` `node/mynode cordoned node/mynode drained ` ``` To impersonate a user, user identifier (UID), group or extra fields, the impersonating user must have the ability to perform the **impersonate**...
---

```
`node/mynode cordoned
node/mynode drained
`
```
To impersonate a user, user identifier (UID), group or extra fields, the impersonating user must
have the ability to perform the **impersonate** verb on the kind of attribute
being impersonated ("user", "uid", "group", etc.). For clusters that enable the RBAC
authorization plugin, the following ClusterRole encompasses the rules needed to
set user and group impersonation headers: