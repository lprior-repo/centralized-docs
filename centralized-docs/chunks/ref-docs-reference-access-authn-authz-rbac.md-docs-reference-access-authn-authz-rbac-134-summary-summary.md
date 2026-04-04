---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#134-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 103
summary: ### `kubectl create clusterrolebinding` Grants a ClusterRole across the entire cluster (all namespaces). Examples: * Across the entire cluster, grant the permissions in the \"cluster-admin\"...
---

### `kubectl create clusterrolebinding`
Grants a ClusterRole across the entire cluster (all namespaces). Examples:
* Across the entire cluster, grant the permissions in the "cluster-admin" ClusterRole to a user named "root":
```
`kubectl create clusterrolebinding root-cluster-admin-binding --clusterrole=cluster-admin --user=root
`
```
* Across the entire cluster, grant the permissions in the "system:node-proxier" ClusterRole to a user named "system:kube-proxy":