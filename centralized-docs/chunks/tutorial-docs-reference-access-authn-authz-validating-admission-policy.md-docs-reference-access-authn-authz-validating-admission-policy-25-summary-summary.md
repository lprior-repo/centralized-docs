---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#25-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 113
summary: Parameter resources allow a policy configuration to be separate from its definition. A policy can define paramKind, which outlines GVK of the parameter resource, and then a policy binding ties a...
---

Parameter resources allow a policy configuration to be separate from its definition.
A policy can define paramKind, which outlines GVK of the parameter resource,
and then a policy binding ties a policy by name (via policyName) to a particular parameter resource via paramRef.
If parameter configuration is needed, the following is an example of a ValidatingAdmissionPolicy
with parameter configuration.
[`validatingadmissionpolicy/policy-with-param.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/policy-with-param.yaml)