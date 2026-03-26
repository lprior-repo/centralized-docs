---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#28-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 95
summary: ### Understanding constrained impersonation Constrained impersonation requires **two separate permissions**: 1. **Permission to impersonate a specific identity** (user, UID, group, service account or...
---

### Understanding constrained impersonation
Constrained impersonation requires **two separate permissions**:
1. **Permission to impersonate a specific identity** (user, UID, group, service account or node)
2. **Permission to perform specific actions at a particular scope when impersonating** (for
example, only `list` and `watch` pods in the `default` namespace)
This means an impersonator can be limited to impersonate another user only for specific operations.