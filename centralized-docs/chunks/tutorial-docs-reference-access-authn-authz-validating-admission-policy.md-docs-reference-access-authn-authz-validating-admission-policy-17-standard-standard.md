---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#17-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 438
summary: Match conditions have access to the same CEL variables as validation expressions. In the event of an error evaluating a match condition the policy is not evaluated. Whether to reject the request is...
---

Match conditions have access to the same CEL variables as validation expressions.
In the event of an error evaluating a match condition the policy is not evaluated. Whether to reject
the request is determined as follows:
1. If **any** match condition evaluated to `false` (regardless of other errors), the API server skips the policy.
2. Otherwise:
* for [`failurePolicy: Fail`](#failure-policy), reject the request (without evaluating the policy).
* for [`failurePolicy: Ignore`](#failure-policy), proceed with the request but skip the policy.### Audit annotations
`auditAnnotations` may be used to include audit annotations in the audit event of the API request.
For example, here is an admission policy with an audit annotation:
[`access/validating-admission-policy-audit-annotation.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/validating-admission-policy-audit-annotation.yaml)![](/images/copycode.svg "Copy access/validating-admission-policy-audit-annotation.yaml to clipboard")
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
- expression: "object.spec.replicas &gt; 50"
messageExpression: "'Deployment spec.replicas set to ' + string(object.spec.replicas)"
auditAnnotations:
- key: "high-replica-count"
valueExpression: "'Deployment spec.replicas set to ' + string(object.spec.replicas)"
`
```
When an API request is validated with this admission policy, the resulting audit event will look like:
```
`# the audit event recorded
{
"kind": "Event",
"apiVersion": "audit.k8s.io/v1",
"annotations": {
"demo-policy.example.com/high-replica-count": "Deployment spec.replicas set to 128"
# other fields
...
}
`
```