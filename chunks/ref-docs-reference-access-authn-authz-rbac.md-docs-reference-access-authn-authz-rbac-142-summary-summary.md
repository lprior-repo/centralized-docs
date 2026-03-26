---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#142-summary
chunk_level: summary
chunk_type: prose
heading: ServiceAccount permissions
token_count: 84
summary: #### Note: Permissions given to the \"default\" service account are available to any pod in the namespace that does not specify a `serviceAccountName`. For example, grant read-only permission within...
---

#### Note:
Permissions given to the "default" service account are available to any pod
in the namespace that does not specify a `serviceAccountName`.
For example, grant read-only permission within "my-namespace" to the "default" service account:
```
`kubectl create rolebinding default-view \\
--clusterrole=view \\
--serviceaccount=my-namespace:default \\
--namespace=my-namespace
`
```