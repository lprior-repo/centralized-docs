---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#126-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 116
summary: ### `kubectl create role` Creates a Role object defining permissions within a single namespace. Examples: * Create a Role named \"pod-reader\" that allows users to perform `get`, `watch` and `list` on...
---

### `kubectl create role`
Creates a Role object defining permissions within a single namespace. Examples:
* Create a Role named "pod-reader" that allows users to perform `get`, `watch` and `list` on pods:
```
`kubectl create role pod-reader --verb=get --verb=list --verb=watch --resource=pods
`
```
* Create a Role named "pod-reader" with resourceNames specified:
```
`kubectl create role pod-reader --verb=get --resource=pods --resource-name=readablepod --resource-name=anotherpod
`
```