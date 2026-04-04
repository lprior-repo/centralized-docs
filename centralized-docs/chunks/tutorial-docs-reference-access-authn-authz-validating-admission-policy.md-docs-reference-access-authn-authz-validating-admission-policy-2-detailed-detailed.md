---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 813
summary: ## Getting Started with Validating Admission Policy Validating Admission Policy is part of the cluster control-plane. You should write and deploy them with great caution. The following describes how...
---

## Getting Started with Validating Admission Policy
Validating Admission Policy is part of the cluster control-plane. You should write and deploy them
with great caution. The following describes how to quickly experiment with Validating Admission Policy.
### Creating a ValidatingAdmissionPolicy
The following is an example of a ValidatingAdmissionPolicy.
[`validatingadmissionpolicy/basic-example-policy.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/basic-example-policy.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/basic-example-policy.yaml to clipboard")
```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
name: "demo-policy.example.com"
spec:
failurePolicy: Fail
matchConstraints:
resourceRules:
- apiGroups: ["apps"]
apiVersions: ["v1"]
operations: ["CREATE", "UPDATE"]
resources: ["deployments"]
validations:
- expression: "object.spec.replicas &lt;= 5"`
```
`spec.validations` contains CEL expressions which use the [Common Expression Language (CEL)](https://github.com/google/cel-spec)
to validate the request. If an expression evaluates to false, the validation check is enforced
according to the `spec.failurePolicy` field.
#### Note:
You can quickly test CEL expressions in [CEL Playground](https://playcel.undistro.io).
To configure a validating admission policy for use in a cluster, a binding is required.
The following is an example of a ValidatingAdmissionPolicyBinding.:
[`validatingadmissionpolicy/basic-example-binding.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/basic-example-binding.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/basic-example-binding.yaml to clipboard")
```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicyBinding
metadata:
name: "demo-binding-test.example.com"
spec:
policyName: "demo-policy.example.com"
validationActions: [Deny]
matchResources:
namespaceSelector:
matchLabels:
environment: test
`
```
When trying to create a deployment with replicas set not satisfying the validation expression, an
error will return containing message:
```
`ValidatingAdmissionPolicy 'demo-policy.example.com' with binding 'demo-binding-test.example.com' denied request: failed expression: object.spec.replicas &lt;= 5
`
```
The above provides a simple example of using ValidatingAdmissionPolicy without a parameter configured.
#### Validation actions
Each `ValidatingAdmissionPolicyBinding` must specify one or more
`validationActions` to declare how `validations` of a policy are enforced.
The supported `validationActions` are:
* `Deny`: Validation failure results in a denied request.
* `Warn`: Validation failure is reported to the request client
as a [warning](/blog/2020/09/03/warnings/).
* `Audit`: Validation failure is included in the audit event for the API request.
For example, to both warn clients about a validation failure and to audit the
validation failures, use:
```
`validationActions: [Warn, Audit]
`
```
`Deny` and `Warn` may not be used together since this combination
needlessly duplicates the validation failure both in the
API response body and the HTTP warning headers.
A `validation` that evaluates to false is always enforced according to these
actions. Failures defined by the `failurePolicy` are enforced
according to these actions only if the `failurePolicy` is set to `Fail` (or not specified),
otherwise the failures are ignored.
See [Audit Annotations: validation failures](/docs/reference/labels-annotations-taints/audit-annotations/#validation-policy-admission-k8s-io-validation-failure)
for more details about the validation failure audit annotation.