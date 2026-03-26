---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#130-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 116
summary: * Create a ClusterRole named \"foo\" with apiGroups specified: ``` `kubectl create clusterrole foo --verb=get,list,watch --resource=replicasets.apps ` ``` * Create a ClusterRole named \"foo\" with...
---

* Create a ClusterRole named "foo" with apiGroups specified:
```
`kubectl create clusterrole foo --verb=get,list,watch --resource=replicasets.apps
`
```
* Create a ClusterRole named "foo" with subresource permissions:
```
`kubectl create clusterrole foo --verb=get,list,watch --resource=pods,pods/status
`
```
* Create a ClusterRole named "foo" with nonResourceURL specified:
```
`kubectl create clusterrole "foo" --verb=get --non-resource-url=/logs/\*
`
```