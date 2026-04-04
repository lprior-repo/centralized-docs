---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#103-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 121
summary: ``` `status: typeChecking: expressionWarnings: - fieldRef: spec.validations[0].expression warning: |- apps/v1, Kind=Deployment: ERROR: &lt;input&gt;:1:7: undefined field 'replicas' | object.replicas...
---

```
`status:
typeChecking:
expressionWarnings:
- fieldRef: spec.validations[0].expression
warning: |-
apps/v1, Kind=Deployment: ERROR: &lt;input&gt;:1:7: undefined field 'replicas'
| object.replicas &gt; 1
| ......^
apps/v1, Kind=ReplicaSet: ERROR: &lt;input&gt;:1:7: undefined field 'replicas'
| object.replicas &gt; 1
| ......^
`
```
Type Checking has the following limitation: