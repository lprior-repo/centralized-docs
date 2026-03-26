---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#82-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 119
summary: #### RoleBinding examples The following examples are `RoleBinding` excerpts that only show the `subjects` section. For a user named `alice@example.com`: ``` `subjects: - kind: User name:...
---

#### RoleBinding examples
The following examples are `RoleBinding` excerpts that only
show the `subjects` section.
For a user named `alice@example.com`:
```
`subjects:
- kind: User
name: "alice@example.com"
apiGroup: rbac.authorization.k8s.io
`
```
For a group named `frontend-admins`:
```
`subjects:
- kind: Group
name: "frontend-admins"
apiGroup: rbac.authorization.k8s.io
`
```
For the default service account in the "kube-system" namespace: