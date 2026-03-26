---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Constrained Impersonation
token_count: 703
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