---
doc_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac
chunk_id: ref/docs-reference-access-authn-authz-rbac.md/docs-reference-access-authn-authz-rbac#131-summary
chunk_level: summary
chunk_type: prose
heading: Default roles and role bindings
token_count: 66
summary: ``` `kubectl create clusterrole \"foo\" --verb=get --non-resource-url=/logs/\* ` ``` * Create a ClusterRole named \"monitoring\" with an aggregationRule specified: ``` `kubectl create clusterrole...
---

```
`kubectl create clusterrole "foo" --verb=get --non-resource-url=/logs/\*
`
```
* Create a ClusterRole named "monitoring" with an aggregationRule specified:
```
`kubectl create clusterrole monitoring --aggregation-rule="rbac.example.com/aggregate-to-monitoring=true"
`
```