---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#20-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 306
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
#### Caution:
The prefix `system:` is reserved for Kubernetes system use, so you should ensure
that you don't have users or groups with names that start with `system:` by
accident.
Other than this special prefix, the RBAC authorization system does not require any format
for usernames.
In Kubernetes, Authenticator modules provide group information.
Groups, like users, are represented as strings, and that string has no format requirements,
other than that the prefix `system:` is reserved.
[ServiceAccounts](/docs/tasks/configure-pod-container/configure-service-account/) have names prefixed
with `system:serviceaccount:`, and belong to groups that have names prefixed with `system:serviceaccounts:`.
#### Note:
* `system:serviceaccount:` (singular) is the prefix for service account usernames.
* `system:serviceaccounts:` (plural) is the prefix for service account groups.