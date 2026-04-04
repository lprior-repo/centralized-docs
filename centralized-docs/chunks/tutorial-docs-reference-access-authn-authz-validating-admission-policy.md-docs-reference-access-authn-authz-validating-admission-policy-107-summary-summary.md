---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#107-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 48
summary: ``` `spec: variables: - name: foo expression: \"'foo' in object.spec.metadata.labels ? object.spec.metadata.labels['foo'] : 'default'\" validations: - expression: variables.foo == 'bar' ` ```
---

```
`spec:
variables:
- name: foo
expression: "'foo' in object.spec.metadata.labels ? object.spec.metadata.labels['foo'] : 'default'"
validations:
- expression: variables.foo == 'bar'
`
```