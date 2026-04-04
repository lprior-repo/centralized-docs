---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#11-detailed
chunk_level: detailed
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 988
summary: ### Type checking When a policy definition is created or updated, the validation process parses the expressions it contains and reports any syntax errors, rejecting the definition if any errors are...
---

### Type checking
When a policy definition is created or updated, the validation process parses the expressions it contains
and reports any syntax errors, rejecting the definition if any errors are found.
Afterward, the referred variables are checked for type errors, including missing fields and type confusion,
against the matched types of `spec.matchConstraints`.
The result of type checking can be retrieved from `status.typeChecking`.
The presence of `status.typeChecking` indicates the completion of type checking,
and an empty `status.typeChecking` means that no errors were detected.
For example, given the following policy definition:
[`validatingadmissionpolicy/typechecking.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/typechecking.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/typechecking.yaml to clipboard")
```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
name: "deploy-replica-policy.example.com"
spec:
matchConstraints:
resourceRules:
- apiGroups: ["apps"]
apiVersions: ["v1"]
operations: ["CREATE", "UPDATE"]
resources: ["deployments"]
validations:
- expression: "object.replicas &gt; 1" # should be "object.spec.replicas &gt; 1"
message: "must be replicated"
reason: Invalid
`
```
The status will yield the following information:
```
`status:
typeChecking:
expressionWarnings:
- fieldRef: spec.validations[0].expression
warning: |-
apps/v1, Kind=Deployment: ERROR: &lt;input&gt;:1:7: undefined field 'replicas'
| object.replicas &gt; 1
| ......^
`
```
If multiple resources are matched in `spec.matchConstraints`, all of matched resources will be checked against.
For example, the following policy definition
[`validatingadmissionpolicy/typechecking-multiple-match.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/typechecking-multiple-match.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/typechecking-multiple-match.yaml to clipboard")
```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
name: "replica-policy.example.com"
spec:
matchConstraints:
resourceRules:
- apiGroups: ["apps"]
apiVersions: ["v1"]
operations: ["CREATE", "UPDATE"]
resources: ["deployments","replicasets"]
validations:
- expression: "object.replicas &gt; 1" # should be "object.spec.replicas &gt; 1"
message: "must be replicated"
reason: Invalid`
```
will have multiple types and type checking result of each type in the warning message.
```
`status:
typeChecking:
expressionWarnings:
- fieldRef: spec.validations[0].expression
warning: |-
apps/v1, Kind=Deployment: ERROR: &lt;input&gt;:1:7: undefined field 'replicas'
| object.replicas &gt; 1
| ......^
apps/v1, Kind=ReplicaSet: ERROR: &lt;input&gt;:1:7: undefined field 'replicas'
| object.replicas &gt; 1
| ......^
`
```
Type Checking has the following limitation:
* No wildcard matching. If `spec.matchConstraints.resourceRules` contains `"\*"` in any of `apiGroups`, `apiVersions` or `resources`,
the types that `"\*"` matches will not be checked.
* The number of matched types is limited to 10. This is to prevent a policy that manually specifying too many types.
to consume excessive computing resources. In the order of ascending group, version, and then resource, 11th combination and beyond are ignored.
* Type Checking does not affect the policy behavior in any way. Even if the type checking detects errors, the policy will continue
to evaluate. If errors do occur during evaluate, the failure policy will decide its outcome.
* Type Checking does not apply to CRDs, including matched CRD types and reference of paramKind. The support for CRDs will come in future release.### Variable composition
If an expression grows too complicated, or part of the expression is reusable and computationally expensive to evaluate,
you can extract some part of the expressions into variables. A variable is a named expression that can be referred later
in `variables` in other expressions.
```
`spec:
variables:
- name: foo
expression: "'foo' in object.spec.metadata.labels ? object.spec.metadata.labels['foo'] : 'default'"
validations:
- expression: variables.foo == 'bar'
`
```