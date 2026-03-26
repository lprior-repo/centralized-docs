---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#3-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 477
summary: To impersonate a user, user identifier (UID), group or extra fields, the impersonating user must have the ability to perform the **impersonate** verb on the kind of attribute being impersonated...
---

To impersonate a user, user identifier (UID), group or extra fields, the impersonating user must
have the ability to perform the **impersonate** verb on the kind of attribute
being impersonated ("user", "uid", "group", etc.). For clusters that enable the RBAC
authorization plugin, the following ClusterRole encompasses the rules needed to
set user and group impersonation headers:
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: impersonator
rules:
- apiGroups: [""]
resources: ["users", "groups", "serviceaccounts"]
verbs: ["impersonate"]
`
```
For impersonation, extra fields and impersonated UIDs are both under the "authentication.k8s.io" `apiGroup`.
Extra fields are evaluated as sub-resources of the resource "userextras". To
allow a user to use impersonation headers for the extra field `scopes` and
for UIDs, a user should be granted the following role:
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: scopes-and-uid-impersonator
rules:
# Can impersonate the user "jane.doe@example.com"
- apiGroups: [""]
resources: ["users"]
verbs: ["impersonate"]
resourceNames: ["jane.doe@example.com"]
# Can impersonate the groups "developers" and "admins"
- apiGroups: [""]
resources: ["groups"]
verbs: ["impersonate"]
resourceNames: ["developers","admins"]
# Can impersonate the extras field "scopes" with the values "view" and "development"
- apiGroups: ["authentication.k8s.io"]
resources: ["userextras/scopes"]
verbs: ["impersonate"]
resourceNames: ["view", "development"]
# Can impersonate the uid "06f6ce97-e2c5-4ab8-7ba5-7654dd08d52b"
- apiGroups: ["authentication.k8s.io"]
resources: ["uids"]
verbs: ["impersonate"]
resourceNames: ["06f6ce97-e2c5-4ab8-7ba5-7654dd08d52b"]
`
```