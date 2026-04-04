---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#19-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 84
summary: When trying to create a deployment with replicas set not satisfying the validation expression, an error will return containing message: ``` `ValidatingAdmissionPolicy 'demo-policy.example.com' with...
---

When trying to create a deployment with replicas set not satisfying the validation expression, an
error will return containing message:
```
`ValidatingAdmissionPolicy 'demo-policy.example.com' with binding 'demo-binding-test.example.com' denied request: failed expression: object.spec.replicas &lt;= 5
`
```
The above provides a simple example of using ValidatingAdmissionPolicy without a parameter configured.