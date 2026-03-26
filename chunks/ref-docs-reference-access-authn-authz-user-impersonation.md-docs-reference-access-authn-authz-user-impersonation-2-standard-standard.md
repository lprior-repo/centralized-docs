---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#2-standard
chunk_level: standard
chunk_type: code
heading: Table of Contents
token_count: 492
summary: #### Note: `Impersonate-Uid` is only available in versions 1.22.0 and higher. An example of the impersonation headers used when impersonating a user with groups: ``` `Impersonate-User:...
---

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