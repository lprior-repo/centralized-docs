---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#114-summary
chunk_level: summary
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 108
summary: validations: - expression: \"variables.exempt || variables.containersToCheck.all(c, c.image.startsWith(variables.environment + '.'))\" messageExpression: \"'only ' + variables.environment + ' images are...
---

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