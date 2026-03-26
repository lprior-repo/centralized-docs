---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#94-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 94
summary: rules for custom resources on these ClusterRoles. To add rules to the `admin`, `edit`, or `view` roles, create a ClusterRole with one or more of the following labels: ``` `metadata: labels:...
---

rules for custom resources on these ClusterRoles. To add rules to the `admin`, `edit`, or `view` roles, create
a ClusterRole with one or more of the following labels:
```
`metadata:
labels:
rbac.authorization.k8s.io/aggregate-to-admin: "true"
rbac.authorization.k8s.io/aggregate-to-edit: "true"
rbac.authorization.k8s.io/aggregate-to-view: "true"
`
```