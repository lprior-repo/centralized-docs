---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#2-detailed
chunk_level: detailed
chunk_type: code
heading: Table of Contents
token_count: 951
summary: #### Note: Prior to 1.11.3 (and 1.10.7, 1.9.11), `( extra name )` could only contain characters which were [legal in HTTP header labels](https://tools.ietf.org/html/rfc7230#section-3.2.6). #### Note:...
---

#### Note:
Prior to 1.11.3 (and 1.10.7, 1.9.11), `( extra name )` could only contain characters which
were [legal in HTTP header labels](https://tools.ietf.org/html/rfc7230#section-3.2.6).
#### Note:
`Impersonate-Uid` is only available in versions 1.22.0 and higher.
An example of the impersonation headers used when impersonating a user with groups:
```
`Impersonate-User: jane.doe@example.com
Impersonate-Group: developers
Impersonate-Group: admins
`
```
An example of the impersonation headers used when impersonating a user with a UID and
extra fields:
```
`Impersonate-User: jane.doe@example.com
Impersonate-Uid: 06f6ce97-e2c5-4ab8-7ba5-7654dd08d52b
Impersonate-Extra-dn: cn=jane,ou=engineers,dc=example,dc=com
Impersonate-Extra-acme.com%2Fproject: some-project
Impersonate-Extra-scopes: view
Impersonate-Extra-scopes: development
`
```
When using `kubectl` set the `--as` command line argument to configure the `Impersonate-User`
header, you can also set the `--as-group` flag to configure the `Impersonate-Group` header，
set the `--as-uid` flag (1.23) to configure `Impersonate-Uid` header, and set the
`--as-user-extra` flag (1.35) to configure `Impersonate-Extra-( extra name )` header.
```
`kubectl drain mynode
`
```
```
`Error from server (Forbidden): User "clark" cannot get nodes at the cluster scope. (get nodes mynode)
`
```
Set the `--as` and `--as-group` flag:
```
`kubectl drain mynode --as=superman --as-group=system:masters
`
```
```
`node/mynode cordoned
node/mynode drained
`
```
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