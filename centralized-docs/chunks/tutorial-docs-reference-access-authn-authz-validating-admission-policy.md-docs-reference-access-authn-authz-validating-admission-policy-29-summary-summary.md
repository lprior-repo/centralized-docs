---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#29-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 120
summary: could also be used as parameter reference. The `spec.validations` fields contain CEL expressions. If an expression evaluates to false, the validation check is enforced according to the...
---

 could also be used as
parameter reference.
The `spec.validations` fields contain CEL expressions. If an expression evaluates to false, the
validation check is enforced according to the `spec.failurePolicy` field.
The validating admission policy author is responsible for providing the ReplicaLimit parameter CRD.
To configure an validating admission policy for use in a cluster, a binding and parameter resource
are created. The following is an example of a ValidatingAdmissionPolicyBinding
that uses a **cluster-wide** param - the same param will be used to validate
every resource request that matches the binding: