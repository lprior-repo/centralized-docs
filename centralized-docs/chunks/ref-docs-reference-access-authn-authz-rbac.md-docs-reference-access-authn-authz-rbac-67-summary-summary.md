---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#67-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 124
summary: The [default user-facing roles](#default-roles-and-role-bindings) use ClusterRole aggregation. This lets you, as a cluster administrator, include rules for custom resources, such as those served by...
---

The [default user-facing roles](#default-roles-and-role-bindings) use ClusterRole aggregation. This lets you,
as a cluster administrator, include rules for custom resources, such as those served by
[CustomResourceDefinitions](/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/)
or aggregated API servers, to extend the default roles.
For example: the following ClusterRoles let the "admin" and "edit" default roles manage the custom resource
named CronTab, whereas the "view" role can perform only read actions on CronTab resources.
You can assume that CronTab objects are named