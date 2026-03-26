---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#140-summary
chunk_level: summary
chunk_type: prose
heading: ServiceAccount permissions
token_count: 116
summary: 1. Grant a role to an application-specific service account (best practice) This requires the application to specify a `serviceAccountName` in its pod spec, and for the service account to be created...
---

1. Grant a role to an application-specific service account (best practice)
This requires the application to specify a `serviceAccountName` in its pod spec,
and for the service account to be created (via the API, application manifest, `kubectl create serviceaccount`, etc.).
For example, grant read-only permission within "my-namespace" to the "my-sa" service account:
```
`kubectl create rolebinding my-sa-view \\
--clusterrole=view \\
--serviceaccount=my-namespace:my-sa \\
--namespace=my-namespace
`
```