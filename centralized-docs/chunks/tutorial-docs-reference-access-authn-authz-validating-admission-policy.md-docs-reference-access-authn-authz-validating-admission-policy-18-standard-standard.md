---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#18-standard
chunk_level: standard
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 194
summary: When an API request is validated with this admission policy, the resulting audit event will look like: ``` `# the audit event recorded { \"kind\": \"Event\", \"apiVersion\": \"audit.k8s.io/v1\",...
---

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
In this example the annotation will only be included if the `spec.replicas` of the Deployment is more than
50, otherwise the CEL expression evaluates to null and the annotation will not be included.
Note that audit annotation keys are prefixed by the name of the `ValidatingAdmissionPolicy` and a `/`. If
another admission controller, such as an admission webhook, uses the exact same audit annotation key, the
value of the first admission controller to include the audit annotation will be included in the audit
event and all other values will be ignored.