---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#31-standard
chunk_level: standard
chunk_type: prose
heading: Default roles and role bindings
token_count: 246
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
* Create a Role named "foo" with apiGroups specified:
```
`kubectl create role foo --verb=get,list,watch --resource=replicasets.apps
`
```
* Create a Role named "foo" with subresource permissions:
```
`kubectl create role foo --verb=get,list,watch --resource=pods,pods/status
`
```
* Create a Role named "my-component-lease-holder" with permissions to get/update a resource with a specific name:
```
`kubectl create role my-component-lease-holder --verb=get,list,watch,update --resource=lease --resource-name=my-component
`
```