---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#147-summary
chunk_level: summary
chunk_type: prose
heading: ServiceAccount permissions
token_count: 69
summary: #### Warning: This allows any application full access to your cluster, and also grants any user with read access to Secrets (or the ability to create any pod) full access to your cluster. ```...
---

#### Warning:
This allows any application full access to your cluster, and also grants
any user with read access to Secrets (or the ability to create any pod)
full access to your cluster.
```
`kubectl create clusterrolebinding serviceaccounts-cluster-admin \\
--clusterrole=cluster-admin \\
--group=system:serviceaccounts
`
```