---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#33-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 113
summary: * `impersonate:arbitrary-node` - Permission to impersonate any specified node * `impersonate:associated-node` - Permission to impersonate only the node to which the impersonator is bound *...
---

* `impersonate:arbitrary-node` - Permission to impersonate any specified node
* `impersonate:associated-node` - Permission to impersonate only the node to which the impersonator is bound
* `impersonate-on:arbitrary-node:&lt;verb&gt;` - Permission to perform `&lt;verb&gt;` when impersonating any node
* `impersonate-on:associated-node:&lt;verb&gt;` - Permission to perform `&lt;verb&gt;` when impersonating the associated node