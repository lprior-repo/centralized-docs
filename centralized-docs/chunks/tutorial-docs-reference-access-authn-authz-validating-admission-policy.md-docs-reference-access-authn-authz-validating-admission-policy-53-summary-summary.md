---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#53-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 123
summary: * **name**: The name of the parameter resource. * **namespace**: The namespace of the parameter resource. * **selector**: A label selector to match multiple parameter resources. *...
---

* **name**: The name of the parameter resource.
* **namespace**: The namespace of the parameter resource.
* **selector**: A label selector to match multiple parameter resources.
* **parameterNotFoundAction**: (Required) Controls the behavior when the specified parameters are not found.
* **Allowed Values**:
* **`Allow`**: The absence of matched parameters is treated as a successful validation by the binding.
* **`Deny`**: The absence of matched parameters is subject to the `failurePolicy` of the policy.
One of `name` or `selector` must be set, but not both.