---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#67-summary
chunk_level: summary
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 124
summary: * 'map': `X + Y` performs a merge where the array positions of all keys in `X` are preserved but the values are overwritten by values in `Y` when the key sets of `X` and `Y` intersect. Elements in...
---

* 'map': `X + Y` performs a merge where the array positions of all keys in `X` are preserved but the values
are overwritten by values in `Y` when the key sets of `X` and `Y` intersect. Elements in `Y` with
non-intersecting keys are appended, retaining their partial order.#### Validation expression examples
|Expression|Purpose|
|`object.minReplicas &lt;= object.replicas &amp;&amp; object.replicas &lt;= object.maxReplicas`|Validate that the three fields defining replicas are ordered appropriately|
|