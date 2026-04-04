---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#12-summary
chunk_level: summary
chunk_type: prose
heading: What Resources Make a Policy
token_count: 120
summary: * A `ValidatingAdmissionPolicyBinding` links the above resources together and provides scoping. If you only want to require an `owner` label to be set for `Pods`, the binding is where you would...
---

* A `ValidatingAdmissionPolicyBinding` links the above resources together and provides scoping.
If you only want to require an `owner` label to be set for `Pods`, the binding is where you would
specify this restriction.
At least a `ValidatingAdmissionPolicy` and a corresponding `ValidatingAdmissionPolicyBinding`
must be defined for a policy to have an effect.
If a `ValidatingAdmissionPolicy` does not need to be configured via parameters, simply leave
`spec.paramKind` in `ValidatingAdmissionPolicy` not specified.