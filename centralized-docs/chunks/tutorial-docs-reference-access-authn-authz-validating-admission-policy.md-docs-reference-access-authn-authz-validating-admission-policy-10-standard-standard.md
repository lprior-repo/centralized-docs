---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#10-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 414
summary: #### Parameter selector In addition to specify a parameter in a binding by `name`, you may choose instead to specify label selector, such that all resources of the policy's `paramKind`, and the...
---

#### Parameter selector
In addition to specify a parameter in a binding by `name`, you may
choose instead to specify label selector, such that all resources of the
policy's `paramKind`, and the param's `namespace` (if applicable) that match the
label selector are selected for evaluation. See [selector](/docs/concepts/overview/working-with-objects/labels/) for more information on how label selectors match resources.
If multiple parameters are found to meet the condition, the policy's rules are
evaluated for each parameter found and the results will be ANDed together.
If `namespace` is provided, only objects of the `paramKind` in the provided
namespace are eligible for selection. Otherwise, when `namespace` is empty and
`paramKind` is namespace-scoped, the `namespace` used in the request being
admitted will be used.
#### Authorization checks
We introduced the authorization check for parameter resources.
User is expected to have `read` access to the resources referenced by `paramKind` in
`ValidatingAdmissionPolicy` and `paramRef` in `ValidatingAdmissionPolicyBinding`.
Note that if a resource in `paramKind` fails resolving via the restmapper, `read` access to all
resources of groups is required.
#### `paramRef`
The `paramRef` field specifies the parameter resource used by the policy. It has the following fields:
* **name**: The name of the parameter resource.
* **namespace**: The namespace of the parameter resource.
* **selector**: A label selector to match multiple parameter resources.
* **parameterNotFoundAction**: (Required) Controls the behavior when the specified parameters are not found.
* **Allowed Values**:
* **`Allow`**: The absence of matched parameters is treated as a successful validation by the binding.
* **`Deny`**: The absence of matched parameters is subject to the `failurePolicy` of the policy.
One of `name` or `selector` must be set, but not both.