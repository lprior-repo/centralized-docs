---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#101-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 116
summary: ``` `apiVersion: admissionregistration.k8s.io/v1 kind: ValidatingAdmissionPolicy metadata: name: \"replica-policy.example.com\" spec: matchConstraints: resourceRules: - apiGroups: [\"apps\"] apiVersions:...
---

```
`apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
name: "replica-policy.example.com"
spec:
matchConstraints:
resourceRules:
- apiGroups: ["apps"]
apiVersions: ["v1"]
operations: ["CREATE", "UPDATE"]
resources: ["deployments","replicasets"]
validations:
- expression: "object.replicas &gt; 1" # should be "object.spec.replicas &gt; 1"
message: "must be replicated"
reason: Invalid`
```