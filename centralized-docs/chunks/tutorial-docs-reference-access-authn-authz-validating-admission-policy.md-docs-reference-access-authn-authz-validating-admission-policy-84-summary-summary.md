---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#84-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 120
summary: `apiVersion: admissionregistration.k8s.io/v1 kind: ValidatingAdmissionPolicy metadata: name: \"demo-policy.example.com\" spec: failurePolicy: Fail matchConstraints: resourceRules: - apiGroups: [\"apps\"]...
---

`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
name: "demo-policy.example.com"
spec:
failurePolicy: Fail
matchConstraints:
resourceRules:
- apiGroups: ["apps"]
apiVersions: ["v1"]
operations: ["CREATE", "UPDATE"]
resources: ["deployments"]
validations:
- expression: "object.spec.replicas &gt; 50"
messageExpression: "'Deployment spec.replicas set to ' + string(object.spec.replicas)"
auditAnnotations:
- key: "high-replica-count"