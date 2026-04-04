---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#98-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 124
summary: ``` `apiVersion: admissionregistration.k8s.io/v1 kind: ValidatingAdmissionPolicy metadata: name: \"deploy-replica-policy.example.com\" spec: matchConstraints: resourceRules: - apiGroups: [\"apps\"]...
---

```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
name: "deploy-replica-policy.example.com"
spec:
matchConstraints:
resourceRules:
- apiGroups: ["apps"]
apiVersions: ["v1"]
operations: ["CREATE", "UPDATE"]
resources: ["deployments"]
validations:
- expression: "object.replicas &gt; 1" # should be "object.spec.replicas &gt; 1"
message: "must be replicated"
reason: Invalid
`
```
The status will yield the following information: