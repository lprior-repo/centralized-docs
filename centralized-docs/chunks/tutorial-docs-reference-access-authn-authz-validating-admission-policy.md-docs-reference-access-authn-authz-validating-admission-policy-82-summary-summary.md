---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#82-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 122
summary: * for [`failurePolicy: Ignore`](#failure-policy), proceed with the request but skip the policy.### Audit annotations `auditAnnotations` may be used to include audit annotations in the audit event of...
---

* for [`failurePolicy: Ignore`](#failure-policy), proceed with the request but skip the policy.### Audit annotations
`auditAnnotations` may be used to include audit annotations in the audit event of the API request.
For example, here is an admission policy with an audit annotation:
[`access/validating-admission-policy-audit-annotation.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/access/validating-admission-policy-audit-annotation.yaml)![](/images/copycode.svg "Copy access/validating-admission-policy-audit-annotation.yaml to clipboard")