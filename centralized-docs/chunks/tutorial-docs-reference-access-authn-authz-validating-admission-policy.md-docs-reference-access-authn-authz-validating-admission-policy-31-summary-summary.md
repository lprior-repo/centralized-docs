---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#31-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 97
summary: ``` `apiVersion: admissionregistration.k8s.io/v1 kind: ValidatingAdmissionPolicyBinding metadata: name: \"replicalimit-binding-test.example.com\" spec: policyName: \"replicalimit-policy.example.com\"...
---

```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicyBinding
metadata:
name: "replicalimit-binding-test.example.com"
spec:
policyName: "replicalimit-policy.example.com"
validationActions: [Deny]
paramRef:
name: "replica-limit-test.example.com"
namespace: "default"
parameterNotFoundAction: Deny
matchResources:
namespaceSelector:
matchLabels:
environment: test
`
```