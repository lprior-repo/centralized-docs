---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#132-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 107
summary: ### `kubectl create rolebinding` Grants a Role or ClusterRole within a specific namespace. Examples: * Within the namespace \"acme\", grant the permissions in the \"admin\" ClusterRole to a user named...
---

### `kubectl create rolebinding`
Grants a Role or ClusterRole within a specific namespace. Examples:
* Within the namespace "acme", grant the permissions in the "admin" ClusterRole to a user named "bob":
```
`kubectl create rolebinding bob-admin-binding --clusterrole=admin --user=bob --namespace=acme
`
```
* Within the namespace "acme", grant the permissions in the "view" ClusterRole to the service account in the namespace "acme" named "myapp":