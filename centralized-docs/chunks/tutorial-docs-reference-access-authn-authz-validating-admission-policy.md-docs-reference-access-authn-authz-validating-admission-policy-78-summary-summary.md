---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#78-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 98
summary: `apiVersion: admissionregistration.k8s.io/v1 kind: ValidatingAdmissionPolicy metadata: name: \"demo-policy.example.com\" spec: failurePolicy: Fail matchConstraints: resourceRules: - apiGroups: [\"\*\"]...
---

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
expression: '!(