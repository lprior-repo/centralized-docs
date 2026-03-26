---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#74-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 109
summary: ``` `rules: - apiGroups: [\"\"] # at the HTTP level, the name of the resource for accessing Pod # objects is \"pods\" resources: [\"pods\"] verbs: [\"get\", \"list\", \"watch\"] - apiGroups: [\"batch\"] # at the...
---

```
`rules:
- apiGroups: [""]
# at the HTTP level, the name of the resource for accessing Pod
# objects is "pods"
resources: ["pods"]
verbs: ["get", "list", "watch"]
- apiGroups: ["batch"]
# at the HTTP level, the name of the resource for accessing Job
# objects is "jobs"
resources: ["jobs"]
verbs: ["get", "list", "watch", "create", "update", "patch", "delete"]
`
```