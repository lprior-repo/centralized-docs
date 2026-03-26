---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#26-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 122
summary: FEATURE STATE: `Kubernetes v1.35 [alpha]`(disabled by default) With the **impersonate** verb, impersonation cannot be limited or scoped. It either grants full impersonation or none at all. Once...
---

FEATURE STATE:
`Kubernetes v1.35 [alpha]`(disabled by default)
With the **impersonate** verb, impersonation cannot be limited or scoped.
It either grants full impersonation or none at all. Once granted permission to
impersonate a user, you can perform any action that user can perform across all
resources and namespaces.
With constrained impersonation, an impersonator can be limited to impersonate another
user only for specific actions on specific resources, rather than being able to perform all actions
that the impersonated user can perform.
This feature is enabled by setting the