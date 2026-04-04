---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#104-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 99
summary: * No wildcard matching. If `spec.matchConstraints.resourceRules` contains `\"\*\"` in any of `apiGroups`, `apiVersions` or `resources`, the types that `\"\*\"` matches will not be checked. * The number...
---

* No wildcard matching. If `spec.matchConstraints.resourceRules` contains `"\*"` in any of `apiGroups`, `apiVersions` or `resources`,
the types that `"\*"` matches will not be checked.
* The number of matched types is limited to 10. This is to prevent a policy that manually specifying too many types.
to consume excessive computing resources. In the order of ascending group, version, and then resource, 11th combination and beyond are ignored.