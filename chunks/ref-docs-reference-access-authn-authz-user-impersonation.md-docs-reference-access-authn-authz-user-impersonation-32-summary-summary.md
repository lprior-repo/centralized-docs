---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#32-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 115
summary: * `impersonate:serviceaccount` - Permission to impersonate a specific service account * `impersonate-on:serviceaccount:&lt;verb&gt;` - Permission to perform `&lt;verb&gt;` when impersonating a...
---

* `impersonate:serviceaccount` - Permission to impersonate a specific service account
* `impersonate-on:serviceaccount:&lt;verb&gt;` - Permission to perform `&lt;verb&gt;` when impersonating a service account#### arbitrary-node and associated-node modes
Use these modes to impersonate nodes. This mode applies when the `Impersonate-User` header value
starts with `system:node:`.
**Verbs:**
* `impersonate:arbitrary-node` - Permission to impersonate any specified node