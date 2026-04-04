---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#7-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 397
summary: ``` `apiVersion: rules.example.com/v1 kind: ReplicaLimit metadata: name: \"replica-limit-test.example.com\" namespace: \"default\" maxReplicas: 3 ` ``` This policy parameter resource limits deployments...
---

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
Notice this binding applies a different parameter to resources which
are not in the `test` environment.
And have a parameter resource:
[`validatingadmissionpolicy/replicalimit-param-prod.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/validatingadmissionpolicy/replicalimit-param-prod.yaml)![](/images/copycode.svg "Copy validatingadmissionpolicy/replicalimit-param-prod.yaml to clipboard")
```
`apiVersion: rules.example.com/v1
kind: ReplicaLimit
metadata:
name: "replica-limit-prod.example.com"
maxReplicas: 100`
```