---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#129-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 126
summary: ### `kubectl create clusterrole` Creates a ClusterRole. Examples: * Create a ClusterRole named \"pod-reader\" that allows user to perform `get`, `watch` and `list` on pods: ``` `kubectl create...
---

### `kubectl create clusterrole`
Creates a ClusterRole. Examples:
* Create a ClusterRole named "pod-reader" that allows user to perform `get`, `watch` and `list` on pods:
```
`kubectl create clusterrole pod-reader --verb=get,list,watch --resource=pods
`
```
* Create a ClusterRole named "pod-reader" with resourceNames specified:
```
`kubectl create clusterrole pod-reader --verb=get --resource=pods --resource-name=readablepod --resource-name=anotherpod
`
```
* Create a ClusterRole named "foo" with apiGroups specified: