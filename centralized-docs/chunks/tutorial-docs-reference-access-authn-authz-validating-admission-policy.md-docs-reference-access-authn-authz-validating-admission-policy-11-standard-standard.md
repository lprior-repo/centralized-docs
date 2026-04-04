---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#11-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 507
summary: #### Note: The `parameterNotFoundAction` field in `paramRef` is **required**. It specifies the action to take when no parameters are found matching the `paramRef`. If not specified, the policy...
---

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