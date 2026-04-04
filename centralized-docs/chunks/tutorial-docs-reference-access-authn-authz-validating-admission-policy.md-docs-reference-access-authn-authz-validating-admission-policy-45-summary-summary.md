---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#45-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 118
summary: * If `optionalNumber` has been defined, then the latter half of the CEL expression will be evaluated, and optionalNumber will be checked to ensure that it contains a value between 5 and 10...
---

* If `optionalNumber` has been defined, then the latter half of the CEL expression will be
evaluated, and optionalNumber will be checked to ensure that it contains a value between 5 and
10 inclusive.#### Per-namespace Parameters
As the author of a ValidatingAdmissionPolicy and its ValidatingAdmissionPolicyBinding,
you can choose to specify cluster-wide, or per-namespace parameters.
If you specify a `namespace` for the binding's `paramRef`, the control plane only
searches for parameters in that namespace.
However, if `namespace`