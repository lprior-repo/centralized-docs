---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#114-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 88
summary: * `system:controller:service-account-controller` * `system:controller:service-controller` * `system:controller:statefulset-controller` * `system:controller:ttl-controller`## Privilege escalation...
---

* `system:controller:service-account-controller`
* `system:controller:service-controller`
* `system:controller:statefulset-controller`
* `system:controller:ttl-controller`## Privilege escalation prevention and bootstrapping
The RBAC API prevents users from escalating privileges by editing roles or role bindings.
Because this is enforced at the API level, it applies even when the RBAC authorizer is not in use.