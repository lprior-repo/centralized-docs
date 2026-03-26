---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#84-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 128
summary: For all authenticated users: ``` `subjects: - kind: Group name: system:authenticated apiGroup: rbac.authorization.k8s.io ` ``` For all unauthenticated users: ``` `subjects: - kind: Group name:...
---

For all authenticated users:
```
`subjects:
- kind: Group
name: system:authenticated
apiGroup: rbac.authorization.k8s.io
`
```
For all unauthenticated users:
```
`subjects:
- kind: Group
name: system:unauthenticated
apiGroup: rbac.authorization.k8s.io
`
```
For all users:
```
`subjects:
- kind: Group
name: system:authenticated
apiGroup: rbac.authorization.k8s.io
- kind: Group
name: system:unauthenticated
apiGroup: rbac.authorization.k8s.io
`
```