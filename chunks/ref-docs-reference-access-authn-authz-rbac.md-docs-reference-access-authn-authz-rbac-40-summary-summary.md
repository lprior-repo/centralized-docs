---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#40-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 76
summary: apiGroup: rbac.authorization.k8s.io roleRef: # \"roleRef\" specifies the binding to a Role / ClusterRole kind: Role #this must be Role or ClusterRole name: pod-reader # this must match the name of the...
---

apiGroup: rbac.authorization.k8s.io
roleRef:
# "roleRef" specifies the binding to a Role / ClusterRole
kind: Role #this must be Role or ClusterRole
name: pod-reader # this must match the name of the Role or ClusterRole you wish to bind to
apiGroup: rbac.authorization.k8s.io
`
```