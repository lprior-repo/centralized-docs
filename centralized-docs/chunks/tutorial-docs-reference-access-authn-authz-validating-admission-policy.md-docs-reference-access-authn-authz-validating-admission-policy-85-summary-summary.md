---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#85-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 115
summary: auditAnnotations: - key: \"high-replica-count\" valueExpression: \"'Deployment spec.replicas set to ' + string(object.spec.replicas)\" ` ``` When an API request is validated with this admission policy,...
---

auditAnnotations:
- key: "high-replica-count"
valueExpression: "'Deployment spec.replicas set to ' + string(object.spec.replicas)"
`
```
When an API request is validated with this admission policy, the resulting audit event will look like:
```
`# the audit event recorded
{
"kind": "Event",
"apiVersion": "audit.k8s.io/v1",
"annotations": {
"demo-policy.example.com/high-replica-count": "Deployment spec.replicas set to 128"
# other fields
...
}
`
```