---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#56-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 118
summary: #### Handling Missing Parameters with `parameterNotFoundAction` When using `paramRef` with a selector, it's possible that no parameters match the selector. The `parameterNotFoundAction` field...
---

#### Handling Missing Parameters with `parameterNotFoundAction`
When using `paramRef` with a selector, it's possible that no parameters match the selector. The `parameterNotFoundAction` field determines how the binding behaves in this scenario.
**Example:**
```
`apiVersion: admissionregistration.k8s.io/v1alpha1
kind: ValidatingAdmissionPolicyBinding
metadata:
name: example-binding
spec:
policyName: example-policy
paramRef:
selector:
matchLabels:
environment: test
parameterNotFoundAction: Allow
validationActions:
- Deny
`
```