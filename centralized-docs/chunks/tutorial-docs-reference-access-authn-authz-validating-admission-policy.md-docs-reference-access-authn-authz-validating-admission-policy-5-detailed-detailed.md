---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 921
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
#### Note:
The `parameterNotFoundAction` field in `paramRef` is **required**. It specifies the action to take when no parameters are found matching the `paramRef`. If not specified, the policy binding may be considered invalid and will be ignored or could lead to unexpected behavior.
* **`Allow`**: If set to `Allow`, and no parameters are found, the binding treats the absence of parameters as a successful validation, and the policy is considered to have passed.
* **`Deny`**: If set to `Deny`, and no parameters are found, the binding enforces the `failurePolicy` of the policy. If the `failurePolicy` is `Fail`, the request is rejected.
Make sure to set `parameterNotFoundAction` according to the desired behavior when parameters are missing.
#### Handling Missing Parameters with `parameterNotFoundAction`
When using `paramRef` with a selector, it's possible that no parameters match the selector. The `parameterNotFoundAction` field determines how the binding behaves in this scenario.
**Example:**
```
`apiVersion: admissionregistration.k8s.io/v1alpha1
kind: ValidatingAdmissionPolicyBinding
metadata:
name: example-binding
spec:
policyName: example-policy
paramRef:
selector:
matchLabels:
environment: test
parameterNotFoundAction: Allow
validationActions:
- Deny
`
```
### Failure Policy
`failurePolicy` defines how mis-configurations and CEL expressions evaluating to error from the
admission policy are handled. Allowed values are `Ignore` or `Fail`.
* `Ignore` means that an error calling the ValidatingAdmissionPolicy is ignored and the API
request is allowed to continue.
* `Fail` means that an error calling the ValidatingAdmissionPolicy causes the admission to fail
and the API request to be rejected.
Note that the `failurePolicy` is defined inside `ValidatingAdmissionPolicy`:
[`validatingadmissionpolicy/failure-policy-ignore.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/failure-policy-ignore.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/failure-policy-ignore.yaml to clipboard")
```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
spec:
...
failurePolicy: Ignore # The default is "Fail"
validations:
- expression: "object.spec.xyz == params.x" `
```