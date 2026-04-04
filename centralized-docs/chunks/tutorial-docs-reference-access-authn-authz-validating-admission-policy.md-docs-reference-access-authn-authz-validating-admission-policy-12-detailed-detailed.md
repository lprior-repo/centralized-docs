---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#12-detailed
chunk_level: detailed
chunk_type: table
heading: API kinds exempt from admission validation
token_count: 865
summary: ``` `spec: variables: - name: foo expression: \"'foo' in object.spec.metadata.labels ? object.spec.metadata.labels['foo'] : 'default'\" validations: - expression: variables.foo == 'bar' ` ``` A...
---

```
`spec:
variables:
- name: foo
expression: "'foo' in object.spec.metadata.labels ? object.spec.metadata.labels['foo'] : 'default'"
validations:
- expression: variables.foo == 'bar'
`
```
A variable is lazily evaluated when it is first referred. Any error that occurs during the evaluation will be
reported during the evaluation of the referring expression. Both the result and potential error are memorized and
count only once towards the runtime cost.
The order of variables are important because a variable can refer to other variables that are defined before it.
This ordering prevents circular references.
The following is a more complex example of enforcing that image repo names match the environment defined in its namespace.
[`access/image-matches-namespace-environment.policy.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/image-matches-namespace-environment.policy.yaml)![](/images/copycode.svg "Copy access/image-matches-namespace-environment.policy.yaml to clipboard")
```
`# This policy enforces that all containers of a deployment has the image repo match the environment label of its namespace.
# Except for "exempt" deployments, or any containers that do not belong to the "example.com" organization (e.g. common sidecars).
# For example, if the namespace has a label of {"environment": "staging"}, all container images must be either staging.example.com/\*
# or do not contain "example.com" at all, unless the deployment has {"exempt": "true"} label.
apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
name: "image-matches-namespace-environment.policy.example.com"
spec:
failurePolicy: Fail
matchConstraints:
resourceRules:
- apiGroups: ["apps"]
apiVersions: ["v1"]
operations: ["CREATE", "UPDATE"]
resources: ["deployments"]
variables:
- name: environment
expression: "'environment' in namespaceObject.metadata.labels ? namespaceObject.metadata.labels['environment'] : 'prod'"
- name: exempt
expression: "'exempt' in object.metadata.labels &amp;&amp; object.metadata.labels['exempt'] == 'true'"
- name: containers
expression: "object.spec.template.spec.containers"
- name: containersToCheck
expression: "variables.containers.filter(c, c.image.contains('example.com/'))"
validations:
- expression: "variables.exempt || variables.containersToCheck.all(c, c.image.startsWith(variables.environment + '.'))"
messageExpression: "'only ' + variables.environment + ' images are allowed in namespace ' + namespaceObject.metadata.name"`
```
With the policy bound to the namespace `default`, which is labeled `environment: prod`,
the following attempt to create a deployment would be rejected.
```
`kubectl create deploy --image=dev.example.com/nginx invalid
`
```
The error message is similar to this.
```
`error: failed to create deployment: deployments.apps "invalid" is forbidden: ValidatingAdmissionPolicy 'image-matches-namespace-environment.policy.example.com' with binding 'demo-binding-test.example.com' denied request: only prod images are allowed in namespace default
`
```
## API kinds exempt from admission validation
There are certain API kinds that are exempt from admission-time validation checks. For example, you can't create a ValidatingAdmissionPolicy that prevents changes to ValidatingAdmissionPolicyBindings.
The list of exempt API kinds is:
* [ValidatingAdmissionPolicies](/docs/reference/kubernetes-api/policy-resources/validating-admission-policy-v1/)
* [ValidatingAdmissionPolicyBindings](/docs/reference/kubernetes-api/policy-resources/validating-admission-policy-binding-v1/)
* MutatingAdmissionPolicies
* MutatingAdmissionPolicyBindings
* [TokenReviews](/docs/reference/kubernetes-api/authentication-resources/token-review-v1/)
* [LocalSubjectAccessReviews](/docs/reference/kubernetes-api/authorization-resources/local-subject-access-review-v1/)
* [SelfSubjectAccessReviews](/docs/reference/kubernetes-api/authorization-resources/self-subject-access-review-v1/)
* [SelfSubjectReviews](/docs/reference/kubernetes-api/authentication-resources/self-subject-review-v1/)