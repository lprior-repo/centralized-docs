---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#21-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 343
summary: If multiple resources are matched in `spec.matchConstraints`, all of matched resources will be checked against. For example, the following policy definition...
---

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