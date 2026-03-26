---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#36-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 91
summary: #### Example: Impersonate a user for specific actions This example shows how to allow a service account to impersonate a user named `jane.doe@example.com`, but only to `list` and `watch` pods in the...
---

#### Example: Impersonate a user for specific actions
This example shows how to allow a service account to impersonate a user named `jane.doe@example.com`,
but only to `list` and `watch` pods in the `default` namespace. You need both a `ClusterRoleBinding`
for the identity permission and a `RoleBinding` for the action permission
**Step 1: Grant permission to impersonate the user identity**