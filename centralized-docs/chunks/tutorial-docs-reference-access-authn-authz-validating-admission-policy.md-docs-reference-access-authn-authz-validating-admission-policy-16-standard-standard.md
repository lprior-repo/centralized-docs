---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#16-standard
chunk_level: standard
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 461
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