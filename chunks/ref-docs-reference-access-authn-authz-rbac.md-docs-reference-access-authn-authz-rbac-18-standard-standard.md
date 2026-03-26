---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#18-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 471
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