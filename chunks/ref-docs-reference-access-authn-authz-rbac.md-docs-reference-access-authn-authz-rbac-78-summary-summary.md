---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#78-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 121
summary: ### Referring to subjects A RoleBinding or ClusterRoleBinding binds a role to subjects. Subjects can be groups, users or...
---

### Referring to subjects
A RoleBinding or ClusterRoleBinding binds a role to subjects.
Subjects can be groups, users or
[ServiceAccounts](/docs/tasks/configure-pod-container/configure-service-account/).
Kubernetes represents usernames as strings.
These can be: plain names, such as "alice"; email-style names, like "bob@example.com";
or numeric user IDs represented as a string. It is up to you as a cluster administrator
to configure the [authentication modules](/docs/reference/access-authn-authz/authentication/)
so that authentication produces usernames in the format you want.