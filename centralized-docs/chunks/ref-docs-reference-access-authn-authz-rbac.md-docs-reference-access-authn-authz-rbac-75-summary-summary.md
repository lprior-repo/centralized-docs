---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#75-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 126
summary: Allow reading a ConfigMap named \"my-config\" (must be bound with a RoleBinding to limit to a single ConfigMap in a single namespace): ``` `rules: - apiGroups: [\"\"] # at the HTTP level, the name of the...
---

Allow reading a ConfigMap named "my-config" (must be bound with a
RoleBinding to limit to a single ConfigMap in a single namespace):
```
`rules:
- apiGroups: [""]
# at the HTTP level, the name of the resource for accessing ConfigMap
# objects is "configmaps"
resources: ["configmaps"]
resourceNames: ["my-config"]
verbs: ["get"]
`
```
Allow reading the resource `"nodes"` in the core group (because a
Node is cluster-scoped, this must be in a ClusterRole bound with a
ClusterRoleBinding to be effective):