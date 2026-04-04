---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#73-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 126
summary: Allow reading/writing Deployments (at the HTTP level: objects with `\"deployments\"` in the resource part of their URL) in the `\"apps\"` API groups: ``` `rules: - apiGroups: [\"apps\"] # at the HTTP...
---

Allow reading/writing Deployments (at the HTTP level: objects with `"deployments"`
in the resource part of their URL) in the `"apps"` API groups:
```
`rules:
- apiGroups: ["apps"]
# at the HTTP level, the name of the resource for accessing Deployment
# objects is "deployments"
resources: ["deployments"]
verbs: ["get", "list", "watch", "create", "update", "patch", "delete"]
`
```
Allow reading Pods in the core API group, as well as reading or writing Job
resources in the `"batch"` API group: