---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#145-summary
chunk_level: summary
chunk_type: prose
heading: ServiceAccount permissions
token_count: 105
summary: 3. Grant a role to all service accounts in a namespace If you want all applications in a namespace to have a role, no matter what service account they use, you can grant a role to the service account...
---

3. Grant a role to all service accounts in a namespace
If you want all applications in a namespace to have a role, no matter what service account they use,
you can grant a role to the service account group for that namespace.
For example, grant read-only permission within "my-namespace" to all service accounts in that namespace:
```
`kubectl create rolebinding serviceaccounts-view \\
--clusterrole=view \\
--group=system:serviceaccounts:my-namespace \\
--namespace=my-namespace
`
```