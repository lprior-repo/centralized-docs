---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#72-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 108
summary: #### Role examples The following examples are excerpts from Role or ClusterRole objects, showing only the `rules` section. Allow reading `\"pods\"` resources in the core [API...
---

#### Role examples
The following examples are excerpts from Role or ClusterRole objects, showing only
the `rules` section.
Allow reading `"pods"` resources in the core
[API Group](/docs/concepts/overview/kubernetes-api/#api-groups-and-versioning):
```
`rules:
- apiGroups: [""]
# at the HTTP level, the name of the resource for accessing Pod
# objects is "pods"
resources: ["pods"]
verbs: ["get", "list", "watch"]
`
```