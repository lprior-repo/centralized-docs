---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#3-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 490
summary: ### Creating a ValidatingAdmissionPolicy The following is an example of a ValidatingAdmissionPolicy. [`validatingadmissionpolicy/basic-example-policy.yaml`...
---

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