---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#62-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 99
summary: ### Aggregated ClusterRoles You can *aggregate* several ClusterRoles into one combined ClusterRole. A controller, running as part of the cluster control plane, watches for ClusterRole objects with an...
---

### Aggregated ClusterRoles
You can *aggregate* several ClusterRoles into one combined ClusterRole.
A controller, running as part of the cluster control plane, watches for ClusterRole
objects with an `aggregationRule` set. The `aggregationRule` defines a label
[selector](/docs/concepts/overview/working-with-objects/labels/) that the controller
uses to match other ClusterRole objects that should be combined into the `rules`
field of this one.