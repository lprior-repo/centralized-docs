---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#23-standard
chunk_level: standard
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 454
summary: ``` `# This policy enforces that all containers of a deployment has the image repo match the environment label of its namespace. # Except for \"exempt\" deployments, or any containers that do not...
---

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