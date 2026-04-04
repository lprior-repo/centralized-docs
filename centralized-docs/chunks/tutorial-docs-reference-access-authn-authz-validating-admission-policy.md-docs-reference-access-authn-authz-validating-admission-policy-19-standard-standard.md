---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#19-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 504
summary: ### Message expression To return a more friendly message when the policy rejects a request, we can use a CEL expression to composite a message with `spec.validations[i].messageExpression`. Similar to...
---

### Message expression
To return a more friendly message when the policy rejects a request, we can use a CEL expression
to composite a message with `spec.validations[i].messageExpression`. Similar to the validation expression,
a message expression has access to `object`, `oldObject`, `request`, `params`, and `namespaceObject`.
Unlike validations, message expression must evaluate to a string.
For example, to better inform the user of the reason of denial when the policy refers to a parameter,
we can have the following validation:
[`access/deployment-replicas-policy.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/deployment-replicas-policy.yaml)![](/images/copycode.svg "Copy access/deployment-replicas-policy.yaml to clipboard")
```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
name: "deploy-replica-policy.example.com"
spec:
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
messageExpression: "'object.spec.replicas must be no greater than ' + string(params.maxReplicas)"
reason: Invalid
`
```
After creating a params object that limits the replicas to 3 and setting up the binding,
when we try to create a deployment with 5 replicas, we will receive the following message.
```
`$ kubectl create deploy --image=nginx nginx --replicas=5
error: failed to create deployment: deployments.apps "nginx" is forbidden: ValidatingAdmissionPolicy 'deploy-replica-policy.example.com' with binding 'demo-binding-test.example.com' denied request: object.spec.replicas must be no greater than 3
`
```
This is more informative than a static message of "too many replicas".
The message expression takes precedence over the static message defined in `spec.validations[i].message` if both are defined.
However, if the message expression fails to evaluate, the static message will be used instead.
Additionally, if the message expression evaluates to a multi-line string,
the evaluation result will be discarded and the static message will be used if present.
Note that static message is validated against multi-line strings.