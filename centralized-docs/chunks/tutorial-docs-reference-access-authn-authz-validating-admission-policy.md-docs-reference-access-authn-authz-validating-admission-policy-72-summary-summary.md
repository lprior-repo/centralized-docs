---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#72-summary
chunk_level: summary
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 115
summary: |Validate the 'details' map is keyed by the items in the 'names' listSet| |`size(object.clusters.filter(c, c.name == object.primary)) == 1`|Validate that the 'primary' property has one and only one...
---

|Validate the 'details' map is keyed by the items in the 'names' listSet|
|`size(object.clusters.filter(c, c.name == object.primary)) == 1`|Validate that the 'primary' property has one and only one occurrence in the 'clusters' listMap|
Read [Supported evaluation on CEL](https://github.com/google/cel-spec/blob/v0.6.0/doc/langdef.md#evaluation)
for more information about CEL rules.
`spec.validation[i].reason` represents a machine-readable description of why this validation failed.