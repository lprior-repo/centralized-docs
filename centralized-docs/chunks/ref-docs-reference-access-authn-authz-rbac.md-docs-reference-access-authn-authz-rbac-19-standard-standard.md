---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#19-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 230
summary: ``` `rules: - apiGroups: [\"\"] # at the HTTP level, the name of the resource for accessing ConfigMap # objects is \"configmaps\" resources: [\"configmaps\"] resourceNames: [\"my-config\"] verbs: [\"get\"] `...
---

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
```
`rules:
- nonResourceURLs: ["/healthz", "/healthz/\*"] # '\*' in a nonResourceURL is a suffix glob match
verbs: ["get", "post"]
`
```