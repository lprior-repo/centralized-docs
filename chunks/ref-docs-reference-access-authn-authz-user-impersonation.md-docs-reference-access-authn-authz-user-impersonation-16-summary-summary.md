---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#16-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 97
summary: ``` `kubectl drain mynode ` ``` ``` `Error from server (Forbidden): User \"clark\" cannot get nodes at the cluster scope. (get nodes mynode) ` ``` Set the `--as` and `--as-group` flag: ``` `kubectl...
---

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