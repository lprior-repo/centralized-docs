---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#51-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 84
summary: #### Authorization checks We introduced the authorization check for parameter resources. User is expected to have `read` access to the resources referenced by `paramKind` in...
---

#### Authorization checks
We introduced the authorization check for parameter resources.
User is expected to have `read` access to the resources referenced by `paramKind` in
`ValidatingAdmissionPolicy` and `paramRef` in `ValidatingAdmissionPolicyBinding`.
Note that if a resource in `paramKind` fails resolving via the restmapper, `read` access to all
resources of groups is required.