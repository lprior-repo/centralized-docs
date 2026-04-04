---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#9-detailed
chunk_level: detailed
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 941
summary: ### Matching requests: `matchConditions` You can define *match conditions* for a `ValidatingAdmissionPolicy` if you need fine-grained request filtering. These conditions are useful if you find that...
---

### Matching requests: `matchConditions`
You can define *match conditions* for a `ValidatingAdmissionPolicy` if you need fine-grained request filtering. These
conditions are useful if you find that match rules, `objectSelectors` and `namespaceSelectors` still
doesn't provide the filtering you want. Match conditions are
[CEL expressions](/docs/reference/using-api/cel/). All match conditions must evaluate to true for the
resource to be evaluated.
Here is an example illustrating a few different uses for match conditions:
[`access/validating-admission-policy-match-conditions.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/validating-admission-policy-match-conditions.yaml)![](/images/copycode.svg "Copy access/validating-admission-policy-match-conditions.yaml to clipboard")
```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
name: "demo-policy.example.com"
spec:
failurePolicy: Fail
matchConstraints:
resourceRules:
- apiGroups: ["\*"]
apiVersions: ["\*"]
operations: ["CREATE", "UPDATE"]
resources: ["\*"]
matchConditions:
- name: 'exclude-leases' # Each match condition must have a unique name
expression: '!(request.resource.group == "coordination.k8s.io" &amp;&amp; request.resource.resource == "leases")' # Match non-lease resources.
- name: 'exclude-kubelet-requests'
expression: '!("system:nodes" in request.userInfo.groups)' # Match requests made by non-node users.
- name: 'rbac' # Skip RBAC requests.
expression: 'request.resource.group != "rbac.authorization.k8s.io"'
validations:
- expression: "!object.metadata.name.contains('demo') || object.metadata.namespace == 'demo'"
`
```
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
In this example the annotation will only be included if the `spec.replicas` of the Deployment is more than
50, otherwise the CEL expression evaluates to null and the annotation will not be included.
Note that audit annotation keys are prefixed by the name of the `ValidatingAdmissionPolicy` and a `/`. If
another admission controller, such as an admission webhook, uses the exact same audit annotation key, the
value of the first admission controller to include the audit annotation will be included in the audit
event and all other values will be ignored.