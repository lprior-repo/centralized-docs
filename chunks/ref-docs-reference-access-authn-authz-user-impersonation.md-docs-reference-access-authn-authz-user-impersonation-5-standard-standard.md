---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#5-standard
chunk_level: standard
chunk_type: prose
heading: Constrained Impersonation
token_count: 253
summary: ## Constrained Impersonation FEATURE STATE: `Kubernetes v1.35 [alpha]`(disabled by default) With the **impersonate** verb, impersonation cannot be limited or scoped. It either grants full...
---

## Constrained Impersonation
FEATURE STATE:
`Kubernetes v1.35 [alpha]`(disabled by default)
With the **impersonate** verb, impersonation cannot be limited or scoped.
It either grants full impersonation or none at all. Once granted permission to
impersonate a user, you can perform any action that user can perform across all
resources and namespaces.
With constrained impersonation, an impersonator can be limited to impersonate another
user only for specific actions on specific resources, rather than being able to perform all actions
that the impersonated user can perform.
This feature is enabled by setting the `ConstrainedImpersonation`
[feature gate](/docs/reference/command-line-tools-reference/feature-gates/#ConstrainedImpersonation).
### Understanding constrained impersonation
Constrained impersonation requires **two separate permissions**:
1. **Permission to impersonate a specific identity** (user, UID, group, service account or node)
2. **Permission to perform specific actions at a particular scope when impersonating** (for
example, only `list` and `watch` pods in the `default` namespace)
This means an impersonator can be limited to impersonate another user only for specific operations.