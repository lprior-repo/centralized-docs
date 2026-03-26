---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#31-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 127
summary: * Does **not** start with `system:serviceaccount:` * Does **not** start with `system:node:` **Verbs:** * `impersonate:user-info` - Permission to impersonate a specific user, group, UID, or extra...
---

* Does **not** start with `system:serviceaccount:`
* Does **not** start with `system:node:`
**Verbs:**
* `impersonate:user-info` - Permission to impersonate a specific user, group, UID, or extra field
* `impersonate-on:user-info:&lt;verb&gt;` - Permission to perform `&lt;verb&gt;` when impersonating a generic user#### ServiceAccount mode
Use this mode to impersonate ServiceAccounts.
**Verbs:**
* `impersonate:serviceaccount` - Permission to impersonate a specific service account