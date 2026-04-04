---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#46-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 122
summary: `namespace` for the binding's `paramRef`, the control plane only searches for parameters in that namespace. However, if `namespace` is not specified in the ValidatingAdmissionPolicyBinding, the API...
---

`namespace` for the binding's `paramRef`, the control plane only
searches for parameters in that namespace.
However, if `namespace` is not specified in the ValidatingAdmissionPolicyBinding, the
API server can search for relevant parameters in the namespace that a request is against.
For example, if you make a request to modify a ConfigMap in the `default` namespace and
there is a relevant ValidatingAdmissionPolicyBinding with no `namespace` set, then the
API server looks for a parameter object in `default`.
This design enables policy configuration that depends on the namespace