---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#94-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 96
summary: This is more informative than a static message of \"too many replicas\". The message expression takes precedence over the static message defined in `spec.validations[i].message` if both are defined....
---

This is more informative than a static message of "too many replicas".
The message expression takes precedence over the static message defined in `spec.validations[i].message` if both are defined.
However, if the message expression fails to evaluate, the static message will be used instead.
Additionally, if the message expression evaluates to a multi-line string,
the evaluation result will be discarded and the static message will be used if present.
Note that static message is validated against multi-line strings.