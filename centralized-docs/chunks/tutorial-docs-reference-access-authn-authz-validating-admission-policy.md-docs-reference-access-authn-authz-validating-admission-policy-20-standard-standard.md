---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#20-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 455
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