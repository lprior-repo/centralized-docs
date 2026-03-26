---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#32-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 91
summary: ``` `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRole metadata: # \"namespace\" omitted since ClusterRoles are not namespaced name: secret-reader rules: - apiGroups: [\"\"] # at the HTTP level,...
---

```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
# "namespace" omitted since ClusterRoles are not namespaced
name: secret-reader
rules:
- apiGroups: [""]
# at the HTTP level, the name of the resource for accessing Secret
# objects is "secrets"
resources: ["secrets"]
verbs: ["get", "watch", "list"]
`
```