---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#16-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 120
summary: * **spec.matchConditions** ([]MatchCondition) *Patch strategy: merge on key `name`* *Map: unique values on key name will be kept during a merge* MatchConditions is a list of conditions that must be...
---

* **spec.matchConditions** ([]MatchCondition)
*Patch strategy: merge on key `name`*
*Map: unique values on key name will be kept during a merge*
MatchConditions is a list of conditions that must be met for a request to be validated. Match conditions filter requests that have already been matched by the rules, namespaceSelector, and objectSelector. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions allowed.
If a parameter object is provided, it can be accessed via the `params` handle in the same manner as validation expressions.