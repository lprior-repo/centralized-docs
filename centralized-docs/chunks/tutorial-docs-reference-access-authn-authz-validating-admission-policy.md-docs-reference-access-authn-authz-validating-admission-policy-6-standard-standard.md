---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#6-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 504
summary: The `spec.paramKind` field of the ValidatingAdmissionPolicy specifies the kind of resources used to parameterize this policy. For this example, it is configured by ReplicaLimit custom resources. Note...
---

The `spec.paramKind` field of the ValidatingAdmissionPolicy specifies the kind of resources used
to parameterize this policy. For this example, it is configured by ReplicaLimit custom resources.
Note in this example how the CEL expression references the parameters via the CEL params variable,
e.g. `params.maxReplicas`. `spec.matchConstraints` specifies what resources this policy is
designed to validate. Note that the native types such like `ConfigMap` could also be used as
parameter reference.
The `spec.validations` fields contain CEL expressions. If an expression evaluates to false, the
validation check is enforced according to the `spec.failurePolicy` field.
The validating admission policy author is responsible for providing the ReplicaLimit parameter CRD.
To configure an validating admission policy for use in a cluster, a binding and parameter resource
are created. The following is an example of a ValidatingAdmissionPolicyBinding
that uses a **cluster-wide** param - the same param will be used to validate
every resource request that matches the binding:
[`validatingadmissionpolicy/binding-with-param.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/binding-with-param.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/binding-with-param.yaml to clipboard")
```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicyBinding
metadata:
name: "replicalimit-binding-test.example.com"
spec:
policyName: "replicalimit-policy.example.com"
validationActions: [Deny]
paramRef:
name: "replica-limit-test.example.com"
namespace: "default"
parameterNotFoundAction: Deny
matchResources:
namespaceSelector:
matchLabels:
environment: test
`
```
Notice this binding applies a parameter to the policy for all resources which
are in the `test` environment.
The parameter resource could be as following:
[`validatingadmissionpolicy/replicalimit-param.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/replicalimit-param.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/replicalimit-param.yaml to clipboard")
```
`apiVersion: rules.example.com/v1
kind: ReplicaLimit
metadata:
name: "replica-limit-test.example.com"
namespace: "default"
maxReplicas: 3
`
```