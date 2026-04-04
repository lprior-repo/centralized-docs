---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#5-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 256
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