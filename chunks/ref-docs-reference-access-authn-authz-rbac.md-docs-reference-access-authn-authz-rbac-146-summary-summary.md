---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#146-summary
chunk_level: summary
chunk_type: prose
heading: ServiceAccount permissions
token_count: 127
summary: 4. Grant a limited role to all service accounts cluster-wide (discouraged) If you don't want to manage permissions per-namespace, you can grant a cluster-wide role to all service accounts. For...
---

4. Grant a limited role to all service accounts cluster-wide (discouraged)
If you don't want to manage permissions per-namespace, you can grant a cluster-wide role to all service accounts.
For example, grant read-only permission across all namespaces to all service accounts in the cluster:
```
`kubectl create clusterrolebinding serviceaccounts-view \\
--clusterrole=view \\
--group=system:serviceaccounts
`
```
5. Grant super-user access to all service accounts cluster-wide (strongly discouraged)
If you don't care about partitioning permissions at all, you can grant super-user access to all service accounts.