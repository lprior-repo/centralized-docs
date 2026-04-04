---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 983
summary: ### Parameter resources Parameter resources allow a policy configuration to be separate from its definition. A policy can define paramKind, which outlines GVK of the parameter resource, and then a...
---

### Parameter resources
Parameter resources allow a policy configuration to be separate from its definition.
A policy can define paramKind, which outlines GVK of the parameter resource,
and then a policy binding ties a policy by name (via policyName) to a particular parameter resource via paramRef.
If parameter configuration is needed, the following is an example of a ValidatingAdmissionPolicy
with parameter configuration.
[`validatingadmissionpolicy/policy-with-param.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/policy-with-param.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/policy-with-param.yaml to clipboard")
```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
name: "replicalimit-policy.example.com"
spec:
failurePolicy: Fail
paramKind:
apiVersion: rules.example.com/v1
kind: ReplicaLimit
matchConstraints:
resourceRules:
- apiGroups: ["apps"]
apiVersions: ["v1"]
operations: ["CREATE", "UPDATE"]
resources: ["deployments"]
validations:
- expression: "object.spec.replicas &lt;= params.maxReplicas"
reason: Invalid`
```
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
This policy parameter resource limits deployments to a max of 3 replicas.
An admission policy may have multiple bindings. To bind all other environments
to have a maxReplicas limit of 100, create another ValidatingAdmissionPolicyBinding:
[`validatingadmissionpolicy/binding-with-param-prod.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/binding-with-param-prod.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/binding-with-param-prod.yaml to clipboard")
```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicyBinding
metadata:
name: "replicalimit-binding-nontest"
spec:
policyName: "replicalimit-policy.example.com"
validationActions: [Deny]
paramRef:
name: "replica-limit-prod.example.com"
namespace: "default"
parameterNotFoundAction: Deny
matchResources:
namespaceSelector:
matchExpressions:
- key: environment
operator: NotIn
values:
- test
`
```