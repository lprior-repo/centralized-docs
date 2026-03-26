---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#34-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 71
summary: #### Note: The `impersonate:associated-node` verb only applies when the impersonator is a service account bound to the node it's trying to impersonate. This is determined by checking if the service...
---

#### Note:
The `impersonate:associated-node` verb only applies when the impersonator is a service account bound to the
node it's trying to impersonate. This is determined by checking if the service account's user info
contains an extra field with key `authentication.kubernetes.io/node-name` that matches the node
being impersonated.