---
doc_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation
chunk_id: ref/docs-reference-access-authn-authz-user-impersonation.md/docs-reference-access-authn-authz-user-impersonation#42-summary
chunk_level: summary
chunk_type: prose
heading: Constrained Impersonation
token_count: 112
summary: name: impersonate-list-watch-pods namespace: default roleRef: apiGroup: rbac.authorization.k8s.io kind: Role name: impersonate-list-watch-pods subjects: - kind: ServiceAccount name: my-controller...
---

name: impersonate-list-watch-pods
namespace: default
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: Role
name: impersonate-list-watch-pods
subjects:
- kind: ServiceAccount
name: my-controller
namespace: default
`
```
Now the `my-controller` service account can impersonate `jane.doe@example.com` to list and watch
pods in the `default` namespace, but **cannot** perform other actions like deleting pods or
accessing resources in other namespaces.