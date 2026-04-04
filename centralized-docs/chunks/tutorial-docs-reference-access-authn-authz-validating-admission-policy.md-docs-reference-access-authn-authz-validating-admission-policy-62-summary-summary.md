---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#62-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 106
summary: * 'object' - The object from the incoming request. The value is null for DELETE requests. * 'oldObject' - The existing object. The value is null for CREATE requests. * 'request' - Attributes of the...
---

* 'object' - The object from the incoming request. The value is null for DELETE requests.
* 'oldObject' - The existing object. The value is null for CREATE requests.
* 'request' - Attributes of the [admission request](/docs/reference/config-api/apiserver-admission.v1/#admission-k8s-io-v1-AdmissionRequest).
* 'params' - Parameter resource referred to by the policy binding being evaluated. The value is
null if `ParamKind` is not specified.