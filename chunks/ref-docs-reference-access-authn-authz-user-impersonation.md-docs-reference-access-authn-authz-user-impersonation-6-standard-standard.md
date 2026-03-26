---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#6-standard
chunk_level: standard
chunk_type: prose
heading: Constrained Impersonation
token_count: 450
summary: ### Impersonation modes Constrained impersonation defines three distinct modes, each with its own set of verbs: #### user-info mode Use this mode to impersonate generic users (not service accounts or...
---

### Impersonation modes
Constrained impersonation defines three distinct modes, each with its own set of verbs:
#### user-info mode
Use this mode to impersonate generic users (not service accounts or nodes). This mode applies when
the `Impersonate-User` header value:
* Does **not** start with `system:serviceaccount:`
* Does **not** start with `system:node:`
**Verbs:**
* `impersonate:user-info` - Permission to impersonate a specific user, group, UID, or extra field
* `impersonate-on:user-info:&lt;verb&gt;` - Permission to perform `&lt;verb&gt;` when impersonating a generic user#### ServiceAccount mode
Use this mode to impersonate ServiceAccounts.
**Verbs:**
* `impersonate:serviceaccount` - Permission to impersonate a specific service account
* `impersonate-on:serviceaccount:&lt;verb&gt;` - Permission to perform `&lt;verb&gt;` when impersonating a service account#### arbitrary-node and associated-node modes
Use these modes to impersonate nodes. This mode applies when the `Impersonate-User` header value
starts with `system:node:`.
**Verbs:**
* `impersonate:arbitrary-node` - Permission to impersonate any specified node
* `impersonate:associated-node` - Permission to impersonate only the node to which the impersonator is bound
* `impersonate-on:arbitrary-node:&lt;verb&gt;` - Permission to perform `&lt;verb&gt;` when impersonating any node
* `impersonate-on:associated-node:&lt;verb&gt;` - Permission to perform `&lt;verb&gt;` when impersonating the associated node
#### Note:
The `impersonate:associated-node` verb only applies when the impersonator is a service account bound to the
node it's trying to impersonate. This is determined by checking if the service account's user info
contains an extra field with key `authentication.kubernetes.io/node-name` that matches the node
being impersonated.