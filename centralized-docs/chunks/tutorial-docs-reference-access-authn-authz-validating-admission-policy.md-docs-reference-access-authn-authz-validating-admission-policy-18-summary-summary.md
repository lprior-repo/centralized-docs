---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#18-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 92
summary: ``` `apiVersion: admissionregistration.k8s.io/v1 kind: ValidatingAdmissionPolicyBinding metadata: name: \"demo-binding-test.example.com\" spec: policyName: \"demo-policy.example.com\" validationActions:...
---

```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicyBinding
metadata:
name: "demo-binding-test.example.com"
spec:
policyName: "demo-policy.example.com"
validationActions: [Deny]
matchResources:
namespaceSelector:
matchLabels:
environment: test
`
```
When trying to create a deployment with replicas set not satisfying the validation expression, an
error will return containing message: