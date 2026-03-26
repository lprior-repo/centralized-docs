---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#16-standard
chunk_level: standard
chunk_type: prose
heading: API objects
token_count: 462
summary: #### Caution: The control plane overwrites any values that you manually specify in the `rules` field of an aggregate ClusterRole. If you want to change or add rules, do so in the `ClusterRole`...
---

#### Caution:
The control plane overwrites any values that you manually specify in the `rules` field of an
aggregate ClusterRole. If you want to change or add rules, do so in the `ClusterRole` objects
that are selected by the `aggregationRule`.
Here is an example aggregated ClusterRole:
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: monitoring
aggregationRule:
clusterRoleSelectors:
- matchLabels:
rbac.example.com/aggregate-to-monitoring: "true"
rules: [] # The control plane automatically fills in the rules
`
```
If you create a new ClusterRole that matches the label selector of an existing aggregated ClusterRole,
that change triggers adding the new rules into the aggregated ClusterRole.
Here is an example that adds rules to the "monitoring" ClusterRole, by creating another
ClusterRole labeled `rbac.example.com/aggregate-to-monitoring: true`.
```
`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
name: monitoring-endpointslices
labels:
rbac.example.com/aggregate-to-monitoring: "true"
# the rules below will be added to the "monitoring" ClusterRole.
rules:
- apiGroups: [""]
resources: ["services", "pods"]
verbs: ["get", "list", "watch"]
- apiGroups: ["discovery.k8s.io"]
resources: ["endpointslices"]
verbs: ["get", "list", "watch"]
`
```
The [default user-facing roles](#default-roles-and-role-bindings) use ClusterRole aggregation. This lets you,
as a cluster administrator, include rules for custom resources, such as those served by
[CustomResourceDefinitions](/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/)
or aggregated API servers, to extend the default roles.
For example: the following ClusterRoles let the "admin" and "edit" default roles manage the custom resource
named CronTab, whereas the "view" role can perform only read actions on CronTab resources.
You can assume that CronTab objects are named `"crontabs"` in URLs as seen by the API server.