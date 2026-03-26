---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#135-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 92
summary: ``` `kubectl create clusterrolebinding kube-proxy-binding --clusterrole=system:node-proxier --user=system:kube-proxy ` ``` * Across the entire cluster, grant the permissions in the \"view\" ClusterRole...
---

```
`kubectl create clusterrolebinding kube-proxy-binding --clusterrole=system:node-proxier --user=system:kube-proxy
`
```
* Across the entire cluster, grant the permissions in the "view" ClusterRole to a service account named "myapp" in the namespace "acme":
```
`kubectl create clusterrolebinding myapp-view-binding --clusterrole=view --serviceaccount=acme:myapp
`
```