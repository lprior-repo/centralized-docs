---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#106-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 88
summary: * Type Checking does not apply to CRDs, including matched CRD types and reference of paramKind. The support for CRDs will come in future release.### Variable composition If an expression grows too...
---

* Type Checking does not apply to CRDs, including matched CRD types and reference of paramKind. The support for CRDs will come in future release.### Variable composition
If an expression grows too complicated, or part of the expression is reusable and computationally expensive to evaluate,
you can extract some part of the expressions into variables. A variable is a named expression that can be referred later
in `variables` in other expressions.