---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#81-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 84
summary: [ServiceAccounts](/docs/tasks/configure-pod-container/configure-service-account/) have names prefixed with `system:serviceaccount:`, and belong to groups that have names prefixed with...
---

[ServiceAccounts](/docs/tasks/configure-pod-container/configure-service-account/) have names prefixed
with `system:serviceaccount:`, and belong to groups that have names prefixed with `system:serviceaccounts:`.
#### Note:
* `system:serviceaccount:` (singular) is the prefix for service account usernames.
* `system:serviceaccounts:` (plural) is the prefix for service account groups.