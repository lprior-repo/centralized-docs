---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#127-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 104
summary: ``` `kubectl create role pod-reader --verb=get --resource=pods --resource-name=readablepod --resource-name=anotherpod ` ``` * Create a Role named \"foo\" with apiGroups specified: ``` `kubectl create...
---

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