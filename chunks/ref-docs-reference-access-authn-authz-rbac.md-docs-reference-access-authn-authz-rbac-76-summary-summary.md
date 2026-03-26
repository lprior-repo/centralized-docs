---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#76-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 89
summary: ``` `rules: - apiGroups: [\"\"] # at the HTTP level, the name of the resource for accessing Node # objects is \"nodes\" resources: [\"nodes\"] verbs: [\"get\", \"list\", \"watch\"] ` ``` Allow GET and POST...
---

```
`rules:
- apiGroups: [""]
# at the HTTP level, the name of the resource for accessing Node
# objects is "nodes"
resources: ["nodes"]
verbs: ["get", "list", "watch"]
`
```
Allow GET and POST requests to the non-resource endpoint `/healthz` and
all subpaths (must be in a ClusterRole bound with a ClusterRoleBinding
to be effective):