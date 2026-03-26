---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#61-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 80
summary: ### Working with `impersonate` verb * If you have existing RBAC rules using the `impersonate` verb, they continue to function when the feature gate is enabled. * When an impersonation request is...
---

### Working with `impersonate` verb
* If you have existing RBAC rules using the `impersonate` verb, they continue
to function when the feature gate is enabled.
* When an impersonation request is made, the API server first checks for
constrained impersonation permissions. If those checks fail, it falls back to checking the
`impersonate` permission.