---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#128-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 79
summary: ``` `kubectl create role foo --verb=get,list,watch --resource=pods,pods/status ` ``` * Create a Role named \"my-component-lease-holder\" with permissions to get/update a resource with a specific name:...
---

```
`kubectl create role foo --verb=get,list,watch --resource=pods,pods/status
`
```
* Create a Role named "my-component-lease-holder" with permissions to get/update a resource with a specific name:
```
`kubectl create role my-component-lease-holder --verb=get,list,watch,update --resource=lease --resource-name=my-component
`
```