---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#32-standard
chunk_level: standard
chunk_type: code
heading: Default roles and role bindings
token_count: 482
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
* Create a ClusterRole named "monitoring" with an aggregationRule specified:
```
`kubectl create clusterrole monitoring --aggregation-rule="rbac.example.com/aggregate-to-monitoring=true"
`
```
### `kubectl create rolebinding`
Grants a Role or ClusterRole within a specific namespace. Examples:
* Within the namespace "acme", grant the permissions in the "admin" ClusterRole to a user named "bob":
```
`kubectl create rolebinding bob-admin-binding --clusterrole=admin --user=bob --namespace=acme
`
```
* Within the namespace "acme", grant the permissions in the "view" ClusterRole to the service account in the namespace "acme" named "myapp":
```
`kubectl create rolebinding myapp-view-binding --clusterrole=view --serviceaccount=acme:myapp --namespace=acme
`
```
* Within the namespace "acme", grant the permissions in the "view" ClusterRole to a service account in the namespace "myappnamespace" named "myapp":
```
`kubectl create rolebinding myappnamespace-myapp-view-binding --clusterrole=view --serviceaccount=myappnamespace:myapp --namespace=acme
`
```