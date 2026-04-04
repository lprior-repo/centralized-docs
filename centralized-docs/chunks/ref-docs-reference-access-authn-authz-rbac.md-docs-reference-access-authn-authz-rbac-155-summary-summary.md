---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#155-summary
chunk_level: summary
chunk_type: prose
heading: Upgrading from ABAC
token_count: 71
summary: ### Parallel authorizers Run both the RBAC and ABAC authorizers, and specify a policy file that contains the [legacy ABAC policy](/docs/reference/access-authn-authz/abac/#policy-file-format): ```...
---

### Parallel authorizers
Run both the RBAC and ABAC authorizers, and specify a policy file that contains
the [legacy ABAC policy](/docs/reference/access-authn-authz/abac/#policy-file-format):
```
`--authorization-mode=...,RBAC,ABAC --authorization-policy-file=mypolicy.json
`
```