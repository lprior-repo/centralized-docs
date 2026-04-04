---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#63-summary
chunk_level: summary
chunk_type: prose
heading: API objects
token_count: 65
summary: #### Caution: The control plane overwrites any values that you manually specify in the `rules` field of an aggregate ClusterRole. If you want to change or add rules, do so in the `ClusterRole`...
---

#### Caution:
The control plane overwrites any values that you manually specify in the `rules` field of an
aggregate ClusterRole. If you want to change or add rules, do so in the `ClusterRole` objects
that are selected by the `aggregationRule`.
Here is an example aggregated ClusterRole: