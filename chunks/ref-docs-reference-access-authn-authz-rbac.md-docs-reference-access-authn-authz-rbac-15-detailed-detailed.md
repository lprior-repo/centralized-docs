---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#15-detailed
chunk_level: detailed
chunk_type: code
heading: Default roles and role bindings
token_count: 923
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
### `kubectl create clusterrolebinding`
Grants a ClusterRole across the entire cluster (all namespaces). Examples:
* Across the entire cluster, grant the permissions in the "cluster-admin" ClusterRole to a user named "root":
```
`kubectl create clusterrolebinding root-cluster-admin-binding --clusterrole=cluster-admin --user=root
`
```
* Across the entire cluster, grant the permissions in the "system:node-proxier" ClusterRole to a user named "system:kube-proxy":
```
`kubectl create clusterrolebinding kube-proxy-binding --clusterrole=system:node-proxier --user=system:kube-proxy
`
```
* Across the entire cluster, grant the permissions in the "view" ClusterRole to a service account named "myapp" in the namespace "acme":
```
`kubectl create clusterrolebinding myapp-view-binding --clusterrole=view --serviceaccount=acme:myapp
`
```
### `kubectl auth reconcile`
Creates or updates `rbac.authorization.k8s.io/v1` API objects from a manifest file.
Missing objects are created, and the containing namespace is created for namespaced objects, if required.
Existing roles are updated to include the permissions in the input objects,
and remove extra permissions if `--remove-extra-permissions` is specified.
Existing bindings are updated to include the subjects in the input objects,
and remove extra subjects if `--remove-extra-subjects` is specified.
Examples:
* Test applying a manifest file of RBAC objects, displaying changes that would be made:
```
`kubectl auth reconcile -f my-rbac-rules.yaml --dry-run=client
`
```
* Apply a manifest file of RBAC objects, preserving any extra permissions (in roles) and any extra subjects (in bindings):
```
`kubectl auth reconcile -f my-rbac-rules.yaml
`
```
* Apply a manifest file of RBAC objects, removing any extra permissions (in roles) and any extra subjects (in bindings):
```
`kubectl auth reconcile -f my-rbac-rules.yaml --remove-extra-subjects --remove-extra-permissions
`
```