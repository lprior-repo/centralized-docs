---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#50-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 79
summary: 's rules are evaluated for each parameter found and the results will be ANDed together. If `namespace` is provided, only objects of the `paramKind` in the provided namespace are eligible for...
---

's rules are
evaluated for each parameter found and the results will be ANDed together.
If `namespace` is provided, only objects of the `paramKind` in the provided
namespace are eligible for selection. Otherwise, when `namespace` is empty and
`paramKind` is namespace-scoped, the `namespace` used in the request being
admitted will be used.